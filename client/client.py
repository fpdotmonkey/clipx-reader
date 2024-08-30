#!/bin/env python3

import socket
import argparse
import sys
import asyncio

import gen.monitor_pb2


class MessageReceivedProtocol:
    def __init__(self, on_connection_lost):
        self.on_con_lost = on_connection_lost
        self.transport = None

    def connection_made(self, transport):
        self.transport = transport

    def datagram_received(self, data, address):
        message = monitor_pb2.ClipxMeasurement.ParseFromString(data)
        print(message)

    def error_received(self, exc):
        print("Error:", exc)

    def connection_lost(self, exc):
        self.on_con_lost.set_result(True)


async def main():
    cli = argparse.ArgumentParser(description="monitor the state of the ClipX device")

    cli.add_argument("--port", default=49359, type=int)
    cli = cli.parse_args()

    print(f"Listening on localhost:{cli.port}")
    loop = asyncio.get_running_loop()
    on_connection_lost = loop.create_future()

    transport, protocol = await loop.create_datagram_endpoint(
        lambda: MessageReceivedProtocol(on_connection_lost),
        local_addr=("localhost", cli.port),
    )

    try:
        await on_connection_lost
    finally:
        transport.close()


if __name__ == "__main__":
    asyncio.run(main())
