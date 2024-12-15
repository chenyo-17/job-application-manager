### What is this
This is a simple program that I use personally to manage local job application files for system engineer and software engineer roles.

### How to use it
1. Create a `config.toml` at `~/.application-manager` and fill in the following parameters with **absolute** path:

``` toml
applications_path = 
system_engineer_template_path = 
software_engineer_template_path = 
```

2. Install the program with `cargo install --path <project-root>`.

3. Record a new job role in CLI with `new-job`.

4. Copy the job description to `job-description.txt` in the generated folder.
   - I don't auto crawl the job description page because many of them use Javascript and I don't bother to start a browser client.






