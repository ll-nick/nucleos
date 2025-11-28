return {
    tasks = {
        test_task = {
            module = nucleos.builtin.file({
                path = "/tmp/test.txt",
            }),
        },
        another_task = {
            module = nucleos.builtin.echo({
                message = "Hello, World!",
            }),
        },
    },
}
