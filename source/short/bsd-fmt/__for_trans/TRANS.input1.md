
It looks good to me. Now let us look at the next part of the program:

### Part 1

```c
/* Global variables */

static int centerP = 0;				/* Try to center lines? */
static size_t goal_length = 0;			/* Target length for output lines */
static size_t max_length = 0;			/* Maximum length for output lines */
static int coalesce_spaces_P = 0;		/* Coalesce multiple whitespace -> ' ' ? */
static int allow_indented_paragraphs = 0;	/* Can first line have diff. ind.? */
static int tab_width = 8;			/* Number of spaces per tab stop */
static size_t output_tab_width = 0;		/* Ditto, when squashing leading spaces */
static const char *sentence_enders = ".?!";	/* Double-space after these */
static int grok_mail_headers = 0;		/* treat embedded mail headers magically? */
static int format_troff = 0;			/* Format troff? */

static int n_errors = 0;			/* Number of failed files. */
static size_t x;				/* Horizontal position in output line */
static size_t x0;				/* Ditto, ignoring leading whitespace */
static size_t pending_spaces;			/* Spaces to add before next word */
static int output_in_paragraph = 0;		/* Any of current para written out yet? */

/* Prototypes */

static void	process_named_file(const char *);
static void	process_stream(FILE *, const char *);
static size_t	indent_length(const char *);
static int	might_be_header(const char *);
static void	new_paragraph(size_t);
static void	output_word(size_t, size_t, const char *, int, int, int);
static void	output_indent(size_t);
static void	center_stream(FILE *, const char *);
static char	*get_line(FILE *);
static void	*xreallocarray(void *, size_t, size_t);
void		usage(void);

#define ERRS(x) (x >= 127 ? 127 : ++x)

/* Here is perhaps the right place to mention that this code is
 * all in top-down order. Hence, |main| comes first.
 */
int
main(int argc, char *argv[])
{
	int ch;			/* used for |getopt| processing */

	(void)setlocale(LC_CTYPE, "");

	/* 1. Grok parameters. */
	while ((ch = getopt(argc, argv, "0123456789cd:hl:mnpst:w:")) != -1) {
		switch (ch) {
		case 'c':
			centerP = 1;
			break;
		case 'd':
			sentence_enders = optarg;
			break;
		case 'l':
			output_tab_width
				= get_positive(optarg, "output tab width must be positive", 1);
			break;
		case 'm':
			grok_mail_headers = 1;
			break;
		case 'n':
			format_troff = 1;
			break;
		case 'p':
			allow_indented_paragraphs = 1;
			break;
		case 's':
			coalesce_spaces_P = 1;
			break;
		case 't':
			tab_width = get_positive(optarg, "tab width must be positive", 1);
			break;
		case 'w':
			goal_length = get_positive(optarg, "width must be positive", 1);
			max_length = goal_length;
			break;
		case '0': case '1': case '2': case '3': case '4': case '5':
		case '6': case '7': case '8': case '9':
			/* XXX  this is not a stylistically approved use of getopt() */
			if (goal_length == 0) {
				char *p;

				p = argv[optind - 1];
				if (p[0] == '-' && p[1] == ch && !p[2])
					goal_length = get_positive(++p, "width must be nonzero", 1);
				else
					goal_length = get_positive(argv[optind]+1,
							"width must be nonzero", 1);
				max_length = goal_length;
			}
			break;
		case 'h':
		default:
			usage();
			/* NOT REACHED */
		}
	}

	argc -= optind;
	argv += optind;

	/* [ goal [ maximum ] ] */
	if (argc > 0 && goal_length == 0 &&
	    (goal_length = get_positive(*argv,"goal length must be positive", 0)) != 0) {
		--argc;
		++argv;
		if (argc > 0 && (max_length = get_positive(*argv,"max length must be positive", 0)) != 0) {
			--argc;
			++argv;
			if (max_length < goal_length)
				errx(1, "max length must be >= goal length");
		}
	}

	if (goal_length == 0)
		goal_length = 65;
	if (max_length == 0)
		max_length = goal_length+10;

	/* 2. Process files. */

	if (argc > 0) {
		while (argc-- > 0)
			process_named_file(*argv++);
	} else {
		process_stream(stdin, "standard input");
	}

	/* We're done. */
	return n_errors;

}

/* Process a single file, given its name.
 */
static void
process_named_file(const char *name)
{
	FILE *f;

	if ((f = fopen(name, "r")) == NULL) {
		warn("%s", name);
		ERRS(n_errors);
	} else {
		process_stream(f, name);
		fclose(f);
	}
}
```

Please translate translate Part 1 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
