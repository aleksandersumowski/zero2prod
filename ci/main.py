import sys

import anyio
import dagger
import random


async def main():
    config = dagger.Config(log_output=sys.stdout)

    async with dagger.Connection(config) as client:
        source = (
            client.container().from_("rust:1.67.1-slim")
            .with_mounted_directory(
                "/src",
                client.host().directory(".", exclude=["ci"])
            )

        )

        test = source.with_workdir("/src").with_exec(["cargo", "test"])
        cmd = ["cargo",
               "install",
               "--path", "."]
        build_dir = test.with_exec(cmd).directory("./target")
        image_ref = await (
            client.container()
            .from_("debian:11-slim")
            .with_directory("/opt/", build_dir)
            .publish(f"ttl.sh/hello-dagger-{random.randint(0, 10000000)}")
        )

        print(f"Published image to: {image_ref}")


anyio.run(main)
