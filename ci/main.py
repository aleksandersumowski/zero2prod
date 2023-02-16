import sys

import anyio
import dagger


async def main():
    config = dagger.Config(log_output=sys.stdout)

    async with dagger.Connection(config) as client:
        source = (
            client.container().
            from_("rust:1.67.1-alpine").
            with_exec(["cargo", "--version"]).
            with_mounted_directory(
                "/src",
                client.host().directory(".", exclude=["ci"])
            )

        )

        test = source.with_workdir("/src").with_exec(["cargo", "test"])
        build_dir = await (
            test.with_exec(["cargo", "build"])
            .directory("./target")
            .export("./target")
        )
        await build_dir.export("./target")
        e = await build_dir.entries()
        print(e)


anyio.run(main)
