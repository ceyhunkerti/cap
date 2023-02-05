# cap

Command line resume generator from `yaml` file.

## Installation

You can get the latest version from [releases](https://github.com/ceyhunkerti/cap/releases)

Example:
```sh
# linux
wget https://github.com/ceyhunkerti/cap/releases/download/v0.1.91/cap-x86_64-unknown-linux-gnu.tar.gz

tar -xvf cap-x86_64-unknown-linux-gnu.tar.gz

./cap --help
```

## Usage

```sh
# see available options
./cap --help

# you need a config file to generate your resumes
# initialize a config file
# ./cap init --help
# this generates `cap.yml` file in the current directory
./cap init

# list available templates
./cap template list

# open your config file (cap.yml) with your editor and update your resume(s)
# vim cap.yml

# list available resumes in your config
./cap resume list --config cap.yml

# generate a resume
./cap resume gen \
    --config cap.yml \
    --name my_resume_1 \
    --out my_resume_2023.pdf
# or
# ./cap resume gen \
#     --config cap.yml \
#     --name my_resume_1 \
#     --template basicplus \
#     --out my_resume_2023.pdf

# checkout your resume `my_resume_2023.pdf` with a pdf viewer
```

See [examples](./examples/) for example config file and the output pdf file.

## Creating Your Custom Resume Template

You can create your `html` resume template and use that template to generate your resume.

- We use [jinja style template engine](https://tera.netlify.app/) to generate html files
- **summary** and **description** fields are markdown enabled so remember put them with `safe` tag in your template like `{{description|safe}}` or `{{summary|safe}}`.
- See an example template in [here](./src/assets/templates/basic.html)
- You can generate your resume with your custom html template with;
```sh
./cap resume gen \
    --config cap.yml \
    --name my_resume_1 \
    --out my_resume_2023.pdf \
    --template `path/to/my_template.html`
```

## Contribution

Contributions are welcome especially new [templates](./src/assets/templates)

### Building
```sh
cargo build
```

### Tests
```sh
cargo test
```
