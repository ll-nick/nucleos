return {
	tasks = {
		test_task = {
			module = "builtin.file",
			options = {
				path = "/tmp/test.txt",
			},
		},
		another_task = {
			module = "builtin.echo",
			options = {
				message = "Hello, World!",
			},
		},
	},
}
