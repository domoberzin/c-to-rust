
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
/* How long is the indent on this line?
 */
static size_t
indent_length(const char *line)
{
	size_t n = 0;

	for (;;) {
		switch(*line++) {
		case ' ':
			++n;
			continue;
		case '\t':
			n = (n / tab_width + 1) * tab_width;
			continue;
		default:
			break;
		}
		break;
	}
	return n;
}

/* Might this line be a mail header?
 * We deem a line to be a possible header if it matches the
 * Perl regexp /^[A-Z][-A-Za-z0-9]*:\s/. This is *not* the same
 * as in RFC whatever-number-it-is; we want to be gratuitously
 * conservative to avoid mangling ordinary civilised text.
 */
static int
might_be_header(const char *line)
{

	if (!isupper((unsigned char)*line++))
		return 0;
	while (isalnum((unsigned char)*line) || *line == '-')
		++line;
	return (*line == ':' && isspace((unsigned char)line[1]));
}

/* Begin a new paragraph with an indent of |indent| spaces.
 */
static void
new_paragraph(size_t indent)
{

	if (x0 > 0)
		putchar('\n');
	x = indent;
	x0 = 0;
	pending_spaces = 0;
	output_in_paragraph = 0;
}

/* Output spaces or tabs for leading indentation.
 */
static void
output_indent(size_t n_spaces)
{

	if (n_spaces == 0)
		return;
	if (output_tab_width) {
		while (n_spaces >= output_tab_width) {
			putchar('\t');
			n_spaces -= output_tab_width;
		}
	}
	while (n_spaces-- > 0)
		putchar(' ');
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
