# Supported markdown formatting

Inline codes are surrounded with backticks.

C program starts with \`main()\`.

becomes

C program starts with `main()`.

--------------------------------------------

# Supported markdown formatting

Code blocks are detected either in two ways: 
- a code fence: a sequence of at least three consecutive backtick characters
  (\`) or tildes (\~). (Tildes and backticks cannot be mixed.)
- an indented code block: composed of one or more indented chunks separated by
  blank lines. An indented chunk is a sequence of non-blank lines, each indented
  four or more spaces.

Both

\`\`\`
int main(int argc, char *argv[]) {
....printf("%s\n", "Hello world!");
}
\`\`\`

and

....int main(int argc, char *argv[]) {
........printf("%s\n", "Hello world!");
....}

will becomes

    int main(int argc, char *argv[]) {
        printf("%s\n", "Hello world!");
    }
