A data science firm has been utilising an AI system called ResumAIte that uses job candidate resumes to predict if the candidate is likely to be skilled enough for the job. The candidates’ skill, Y
, is a binary random variable, which is equal to 1
, if the job candidate can complete the coding task. The system resumAIte generates a score C(x)∈[0,1]
 based on a number of covariates x
 where C(x)=1
 and C(x)=0
 represent acceptance and non-acceptance of job candidates for the roles respectively.

One of the covariates includes the University each candidate last graduated from. In this context, let’s consider that University (e.g. less or more prestigious) is a sensitive or protected attribute, A
, that takes the value A=0
 for University 1 and A=1
 for University 2. Data on successful coding task completion and job acceptance score was collected from the system and the corresponding confusion table is shown below:

|Task|Y| A=0 University 1 C(x)=1 | A=0 University 1 C(x)=0 |A=1 University 2 C(x)=1 |A=1 University 2 C(x)=2 |
|:-|:-|:-|:-|:-|:-|
|Completed coding task|1|55|12|31|44|
|Completed coding task|0|28|180|17|302|

Q1. With respect to the protected attribute, A
, determine if the AI system ‘ResumAIte’ is a fair algorithm using the fairness criteria positive predictive parity.
o




