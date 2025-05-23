Nama : Arya Raditya Kusuma
NPM : 2306215816

## Understanding Questions

> a. How much data your publisher program will send to the message broker in one run?

The amount of data a publisher program sends to the message broker in a single run is not a fixed quantity and can vary dramatically based on the specific application's design and purpose. For instance, a publisher designed for IoT scenarios might send very small messages, perhaps only a few bytes or kilobytes, representing sensor readings or status updates at frequent intervals. Conversely, a publisher tasked with processing batch jobs or transferring files could send significantly larger messages, potentially megabytes or even gigabytes, though very large messages are often broken down or handled via alternative mechanisms like claim checks. Ultimately, the publisher's internal logic, the nature of the information being conveyed, and any configured message size limits on the broker itself will dictate the volume of data transmitted per message and per run. Therefore, there's no universal answer; it is entirely dependent on the publisher's implementation and its role within the overall system.

> b. The URL of: amqp://guest:guest@localhost:5672 is the same as in the subscriber program, what does it mean?

When both the publisher program and the subscriber program use the identical connection URL, amqp://guest:guest@localhost:5672, it signifies that both applications are configured to connect to the exact same instance of an AMQP message broker. This shared URL explicitly tells both programs to authenticate with the username "guest" and the password "guest" to a broker running on the localhost (the same machine as the programs) and listening on port 5672. Consequently, messages published by the publisher program using this connection will be sent to this specific broker instance. Similarly, the subscriber program, by using the same URL, will connect to that very same broker to listen for and consume messages, provided it's subscribed to the relevant queues or exchanges where the publisher sends its messages. This shared "address" is fundamental for them to communicate, as it ensures they are interacting with a common intermediary facilitating their message exchange.


## Running RabbitMQ
![img1](img1.png)

## Sending and processing event
![img2](img2.png)
![img3](img3.png)

## Monitoring chart based on publisher
![img4](img4.png)
Explanation: When I run my publisher program, I observe spikes appearing on the message rates chart in the RabbitMQ management interface. These spikes are a direct result of my publisher actively sending messages to the RabbitMQ broker. Each time I execute my publisher, it transmits one or more messages, which the broker then processes. This sudden influx of messages causes a temporary surge in the "Publish" rate, visually represented as a spike on the graph. Therefore, repeatedly running my publisher program leads to a series of these spikes, each corresponding to a burst of messages I sent.