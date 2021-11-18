## tribonacci

This is a small project originally written in C# to fulfil a coding task for a (successful) job application. I've since
ported it to Rust and updated this README. Some design decisions have been retained from the original version whereas
others have been changed to be more idiomatic to Rust. I've also added a CI / CD pipeline for running the tests,
building the docker image, and deploying to Kubernetes. The original task was to create a dockerised webservice that
calculates [tribonacci numbers](https://en.wikipedia.org/wiki/Generalizations_of_Fibonacci_numbers#Tribonacci_numbers).
This README file contains information about the service and how the service has been implemented.

### Contents

* [Using the service](#using-the-service)
* [Installation Instructions](#installation-instructions)
    * [Building the Service with Cargo](#building-the-service-with-cargo)
    * [Building the Service with Docker](#building-the-service-with-docker)
* [Implementation Notes](#implementation-notes)
    * [Algorithm](#algorithm)
    * [Automated Tests](#automated-tests)
    * [Dockerfile](#dockerfile)
    * [CI / CD Pipeline](#cicd-pipeline)
* [Reflections](#reflections)

### Using the service

The service is available at http://tribonacci.whoggarth.org (at the time of writing). You may test out the service
there. Alternatively, you can download and run the service yourself following the installation instructions contained in
this README file.

The service responds to GET requests at the following path:

/tribonacci/&lt;n&gt;

&lt;n&gt; indicates which number from the tribonacci sequence is being requested. n must be an integer from 1 to 40. (A
larger number is not accepted as the result would be too large for an unsigned 32-bit integer). For example if the
service is running locally and we want to know the 10th number in the sequence we would use the following url:

http://localhost:8000/tribonacci/10

We will get back a page (with an HTTP status code of 200 OK) containing the result:

44

The result is always a plain integer. In the case of a validation error, the service will return an error message in the
following json format (with an HTTP status code of 400 Bad Request):

{"Err":"&lt;error message&gt;"}

&lt;error message&gt; will be a string detailing the error e.g.

{"Err":"can not parse \"ten\" to a u32"}

### Installation Instructions

The service can be built and run using either Cargo or Docker. The instructions for each approach are below:

#### Building the Service with Cargo

If you have rust development environment already set up you can simply build and run the project using ``cargo run``.
You can test the service is up and running by going to the following url:

http://localhost:8000/tribonacci/10

#### Building the Service with Docker

In the top directory of the project, i.e. where the Dockerfile is, run the following command:

`docker build -t tribonacci .`

This will build the project as a docker image. You can then run it on your machine with the following command:

`docker run -d -p 8000:8000 tribonacci`

You can test the service is up and running by going to the following url:

http://localhost:8000/tribonacci/10

### Implementation Notes

#### Algorithm

The tribonacci sequence starts with three numbers 0, 0, and 1. (There are variants that have slightly different starting
values). Subsequent values in the sequence are the sum of the three previous values e.g.

0 + 0 + 1 = 1  
0 + 1 + 1 = 2  
1 + 1 + 2 = 4  
...

The first ten items in the sequence are as follows:

0, 0, 1, 1, 2, 4, 7, 13, 24, 44...

There are essentially two approaches to calculating the nth tribonacci number: recursive and iterative.

Using the recursive approach a function that accepts the three previous values and a count of iterations remaining,
repeatedly calls itself. Each recursion calculates the next number in the sequence, passes that result with its two
previous results and a decremented iteration count. Once the desired number of iterations has been reached the function
returns.

Using the iterative approach a tuple is used to store the previous values and with each iteration of the loop these
values are shifted along with the newly calculated result taking the last position.

I chose the iterative approach as this results in less pressure on the stack. (NB Neither .NET nor Rust guarantee tail
call optimisation, although it might happen in practice).

#### Automated Tests

There are both unit and integration tests included in the project, and these can be run using ``cargo test``. There is
some overlap in what they test as they are mostly to demonstrate the two different types of test. They are also run as
part of the CI/CD pipeline.

#### Dockerfile

The docker file uses the official Rust image for building, and "scratch" as the base for the final image. Link Time
Optimisation (LTO) is also enabled in Cargo.toml to help further reduce the final image size.

#### CI/CD Pipeline

GitHub Actions is used to test, build and deploy the service on each commit. On each build the docker image is tagged
with the relevant commit hash, and a basic Helm chart is used to deploy the new image to Kubernetes. The Helm chart
includes manifests for the deployment, service and ingress. The helm chart is mainly used as a simple way to dynamically
update the image tag used in the deployment.

### Reflections

Several years ago, when I first tackled this task, I knew nothing about Docker and had to learn about it from scratch.
This time round I didn't have to learn or use anything new per se, but it has been useful as practice in putting
together a "complete" Rust project. There are some limitations retained from the original C# version, such as the use of
32-bit integers rather than a using a big integer type, but having said that, feedback is always welcome!
