# Super Koi\n\n## What is Super Koi?\n\nSuper Koi is a handy tool allowing you to interact with ChatGPT via the command line. It even facilitates various complex tasks by allowing ChatGPT to execute commands on your workstation to assist you.\n\n## Demo\n\n\nhttps://user-images.githubusercontent.com/24887625/227695739-de3d96ff-94cc-42bf-80a9-a0d292d2fae9.mp4\n\n\n## How do I use Super Koi?\n\nSuper Koi is a simple tool to use. You can use it by running the following command:\n\n```bash\nkoi -a <OPENAI_API_KEY>\n```\n\nThis will open up a prompt for you to start asking questions.\n\nYou can edit the configuration file to pre-fill the API key so you don't have to enter it every time, allowing you to just run `koi` to start using it.\n\n## Installation\n\n### Cargo\nTo install Super Koi using Cargo, you can run the following command in your terminal:\n\n```bash\ncargo install koios\n```\nYou will need to have [Rust](https://www.rust-lang.org/tools/install) installed in order to install Super Koi from source.\n\n### From Source\nTo install Super Koi from source, you can run the following commands in your terminal:\n\n```bash\ncd koi\ngit clone https://github.com/yqwtf/super-koi.git\ncargo install --path .\n```\nYou will need to have [Rust](https://www.rust-lang