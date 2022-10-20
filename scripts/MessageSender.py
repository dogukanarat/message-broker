from sqlite3 import Timestamp
import time
import zmq
import sys

# main function
def main():
    # create a context
    context = zmq.Context()
    print("Context created")

    # create a socket
    socket = context.socket(zmq.REQ)
    print("Socket created")

    # connect to the server
    socket.connect("tcp://localhost:5555")
    print("Connected to server")

    # send the message
    timestamp = 0
    dummy_message = "Hello World. Timestap: "

    while True:
        timestamp += 1
        
        message = dummy_message + str(timestamp)

        socket.send_string(message)

        print("Message sent: " + message)

        received_message = socket.recv_string()

        print("Message received: " + received_message)

        # sleep for 1 second
        time.sleep(1)

    socket.send_string(dummy_message)

    # close the socket
    socket.close()

    # close the context
    context.term()


# call the main function
if __name__ == "__main__":

    # print welcome message
    print("Message Sender")

    # call the main function
    main()
