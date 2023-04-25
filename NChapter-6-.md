# A New Adventure: Handling External Commands and Dependencies

Hello, Rustaceans! Welcome back to our journey of learning Rust and building an open-source tool. In this episode, we're going to explore how to handle external commands and dependencies. Let's get started!

## Dealing with External Commands

In our previous adventures, we've worked with environment variables, command handling, and fetching default values. Now, we want to ensure that the user has all the necessary tools installed on their system to run our instruction commands.

![](https://i.imgur.com/kEYunGr.png)

We'll start by adding a new function called `handle_external_commands`:

```rust
fn handle_external_commands(command: &Instructions) {
    // (implementation details will be explained below)
}
```

This function will take a reference to the `Instructions` struct and check if the required external commands are installed on the user's system. If not, it will print the list of installation commands.

### Updating the Main Function

Next, let's update the main function to use the `handle_external_commands` function. We'll also add error handling for the deserialization of the `Instructions` struct:

```rust
let command: Result<Instructions, serde_json::Error> =
    serde_json::from_str(&response.choices[0].message.content);

// match the command and if error is found send request again
match command {
    Ok(command) => {
        handle_external_commands(&command);
        println!("{}", command.instruction_commands[0]);
    }
    Err(_) => {
        let response: ApiResponse = get_response(query, tokens).await?;
        let command: Instructions =
            serde_json::from_str(&response.choices[0].message.content).expect(
                "Error in parsing the response, Please try again with a different query",
            );
        handle_external_commands(&command);
        println!("{}", command.instruction_commands[0]);
    }
}
```

### Implementing the `handle_external_commands` Function

Now, let's dive into the implementation of the `handle_external_commands` function. We'll iterate through the `external_commands` and check if they're installed using the `which` command. If not, we'll print the corresponding installation commands:

```rust
fn handle_external_commands(command: &Instructions) {
    let mut found_one = false;
    command
        .external_commands
        .iter()
        .enumerate()
        .for_each(|(index, tool)| {
            let output = Command::new("which").arg(tool.trim()).output();
            match output {
                Ok(output) => {
                    if !output.status.success() {
                        if !found_one {
                            println!("Run the following commands to install the required tools:");
                            found_one = true;
                        }
                        println!("{}", command.external_install[index].to_string())
                    }
                }
                Err(_) => {}
            }
        });
}
```

The `handle_external_commands` function starts by initializing a `found_one` variable to `false`. This variable will be used to print the "Run the following commands to install the required tools:" message only once.

Then, it iterates through the `external_commands` using `enumerate` and `for_each`. For each command, it checks if the command is installed using the `which` command. If the command is not installed, it prints the corresponding installation command from the `external_install` vector.





# Improving Error Handling and Response Parsing

Hello again, Rustaceans! In this episode, we will improve our error handling and response parsing when fetching instruction commands. Let's dive in!

## Introducing the `handle_request` Function

We'll start by introducing a new function called `handle_request`. This function will handle the request to fetch instruction commands and will make three attempts to parse the response before giving up:

![](https://i.imgur.com/UENOy53.png)

```rust
async fn handle_request(query: String, tokens: u32) -> Result<(), Box<dyn Error>> {
    let mut command = None;
    for _ in 0..3 {
        let response: ApiResponse = get_response(query.clone(), tokens).await?;
        if let Ok(parsed_command) =
            serde_json::from_str::<Instructions>(&response.choices[0].message.content)
        {
            command = Some(parsed_command);
            break;
        }
    }

    match command {
        Some(command) => {
            handle_external_commands(&command);
            println!("{}", command.instruction_commands[0]);
        }
        None => {
            println!("Error in parsing the response, Please try again with a different query");
        }
    }
    Ok(())
}
```

The `handle_request` function takes two arguments: `query` and `tokens`. Inside the function, we initialize a mutable variable `command` to `None`. We then loop three times to get the response from the `get_response` function and attempt to parse the response. If the parsing is successful, we set `command` to `Some(parsed_command)` and break out of the loop.

After the loop, we match the `command` variable. If there's a valid command, we proceed with handling external commands and printing the instruction command. If the command is still `None`, we print an error message suggesting the user try again with a different query.

## Updating the `main` Function

Now, let's update the `main` function to use the `handle_request` function:

```rust
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::from_args();
    let tokens = 100;

    // Code for setting up query and OS variables...

    handle_request(query, tokens).await?;
}
```

We've replaced the previous response handling code in the `main` function with a single call to `handle_request`, simplifying the code and improving readability.

## Refining the `handle_external_commands` Function

Lastly, let's make a small change to the `handle_external_commands` function to ensure that the index is in bounds when accessing the `external_install` vector:

```rust
fn handle_external_commands(command: &Instructions) {
    // ... (previous code)

    command
        .external_commands
        .iter()
        .enumerate()
        .for_each(|(index, tool)| {
            // ... (previous code)

            if !output.status.success() {
                if !found_one {
                    println!("Run the following commands to install the required tools:");
                    found_one = true;
                }
                println!(
                    "{}",
                    command
                        .external_install
                        .get(index)
                        .expect("Index out of bounds")
                );
            }

            // ... (previous code)
        });
}
```

We've replaced the direct index access with the `get` method, which returns an `Option`. We then use the `expect` method to handle the case when the index is out of bounds, providing a more meaningful error message.


## Recape of everything we have done till now

### Here is what out structure looks like now

![](https://i.imgur.com/MVCQqzE.png)

---
</br>

### High level overview 




![](https://i.imgur.com/hkuPETl.png)


Here's the step-by-step explanation of the high-level overview flowchart:

1. **Start**: The program begins execution.
2. **Load environment variables**: The program loads the environment variables from the .env file (if available) using the `dotenv` crate.
3. **Parse command line arguments**: The program parses the command line arguments provided by the user using the `clap` crate and stores them in the `Args` struct.
4. **Config command**: If the user selected the "Config" command, the program proceeds to update the .env file or display system information.
    - **Update .env file**: If the user provided tokens as an argument, the program appends the tokens to the .env file.
    - **Display system information**: If the user used the `-d` or `--display` flag, the program prints the operating system and the default token value.
5. **Search command**: If the user selected the "Search" command, the program proceeds to handle the user's query.
    - **Handle request**: The program starts the process of handling the user's query, which involves making API calls, parsing the response, and executing external commands if needed.
        - **Call the API**: The program sends a request to the OpenAI API with the user's query and the token count.
        - **Parse API response**: After receiving the API response, the program attempts to parse the response into an `Instructions` struct.
        - **Execute external commands**: If the response contains external commands, the program checks if they are installed and suggests installation instructions if necessary.
        - **Display instructions**: Finally, the program prints the instructions obtained from the API response for the user to follow.

This high-level overview provides a summary of how the program executes different tasks based on the command provided by the user and interacts with the OpenAI API to provide the desired results.

## Wrapping Up

In this episode, we've improved error handling and response parsing by introducing the `handle_request` function, simplifying the `main` function, and refining the `handle_external_commands` function. These changes make our code more robust and user-friendly.

And that's all for today's episode! As always, stay tuned for more exciting Rust adventures. Happy coding! 🦀🚀

