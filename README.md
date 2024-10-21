
# Larn

Larn is a command-line tool designed to help software engineers improve their coding skills through AI-driven feedback. It analyzes code changes, identifies areas of improvement, and generates personalized programming exercises based on those weaknesses. 

## Features

- **Code Review Automation**: Analyzes code changes and provides feedback on quality, bugs, performance, readability, and security.
- **Personalized Learning**: Identifies areas of struggle based on the feedback and creates programming challenges to help users improve.
- **AI Integration**: Uses GPT-4o to generate detailed feedback and assignments.

## Requirements

To run this project, you will need:

- **Rust** (version 1.56 or higher)
- **Redis** (for storing feedback)
- **Git** (for tracking code changes)
- A **Git repository initialized** in your project directory.
- An OpenAI API key (for code analysis and challenge generation)

## Installation

1. **Install Larn via Cargo:**

   Run the following command to install Larn globally:

   ```bash
   cargo install larn
   ```

2. **Set up Redis**:

   Make sure Redis is installed and running on your local machine. 

3. **Initialize a Git repository**:

   Larn requires an initialized Git repository to track code changes. In the root directory of your project, run:

   ```bash
   git init
   ```

4. **Set up GPT-4 API**:

   You need to have access to the GPT-4 API. Set up your environment variable for the API key:

   ```bash
   export OPENAI_API_KEY="your-gpt4-api-key"
   ```

## Usage

Larn provides two main commands: `add` and `learn`.

### `add` Command

This command collects code changes from the current directory and retrieves feedback.

```bash
larn add
```

What happens:
- The tool scans the directory for code changes (supported languages: `.rs`, `.py`, `.js`, `.cpp`, etc.).
- It runs `git diff` to capture modifications (hence, Git initialization is required).
- The code changes are sent to GPT-4o for review.
- Feedback is stored in Redis.

### `learn` Command

This command generates programming challenges based on previous feedback.

```bash
larn learn
```

What happens:
- It retrieves past feedback from Redis.
- It identifies the areas where you're struggling based on feedback.
- GPT-4 generates programming exercises to improve those areas.
- A text file with assignments is created in the root directory.
- Older feedback is deleted from redis everytime yhis is run.

### Example Workflow

1. Make code changes in your project.
2. Run the `add` command to get feedback on those changes.
3. Once it's time to end the day run the `learn` command to receive personalized exercises to improve your skills.

## Example Commands

```bash
# Get feedback on code changes
larn add

# Generate a learning exercise based on feedback
larn learn
```

## Configuration

Larn is configured to work with multiple programming languages, including:

- Rust
- Python
- JavaScript
- C++
- Java
- TypeScript
- Go
- Ruby
- PHP

You can modify the supported file extensions in the source code if needed.

## Contributing

Contributions are welcome! If you find any bugs or have feature requests, please open an issue or submit a pull request.

## License

Larn is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Happy coding with Larn!
