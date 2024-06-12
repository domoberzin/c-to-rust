
It looks good to me. Now let us look at the next part of the program:

### Part 2

```c
/*
 * Print "standard" escape characters 
 */
static int
print_escape(const char *str)
{
	const char *start = str;
	int value = 0;
	int c;

	str++;

	switch (*str) {
	case '0': case '1': case '2': case '3':
	case '4': case '5': case '6': case '7':
		for (c = 3; c-- && isodigit(*str); str++) {
			value <<= 3;
			value += octtobin(*str);
		}
		putchar(value);
		return str - start - 1;
		/* NOTREACHED */

	case 'x':
		str++;
		for (c = 2; c-- && isxdigit((unsigned char)*str); str++) {
			value <<= 4;
			value += hextobin(*str);
		}
		putchar (value);
		return str - start - 1;
		/* NOTREACHED */

	case '\\':			/* backslash */
		putchar('\\');
		break;

	case '\'':			/* single quote */
		putchar('\'');
		break;

	case '"':			/* double quote */
		putchar('"');
		break;

	case 'a':			/* alert */
		putchar('\a');
		break;

	case 'b':			/* backspace */
		putchar('\b');
		break;

	case 'e':			/* escape */
#ifdef __GNUC__
		putchar('\e');
#else
		putchar(033);
#endif
		break;

	case 'f':			/* form-feed */
		putchar('\f');
		break;

	case 'n':			/* newline */
		putchar('\n');
		break;

	case 'r':			/* carriage-return */
		putchar('\r');
		break;

	case 't':			/* tab */
		putchar('\t');
		break;

	case 'v':			/* vertical-tab */
		putchar('\v');
		break;

	case '\0':
		warnx("null escape sequence");
		rval = 1;
		return 0;

	default:
		putchar(*str);
		warnx("unknown escape sequence `\\%c'", *str);
		rval = 1;
	}

	return 1;
}

static char *
mklong(const char *str, int ch)
{
	static char *copy;
	static int copysize;
	int len;	

	len = strlen(str) + 2;
	if (copysize < len) {
		char *newcopy;
		copysize = len + 256;

		newcopy = realloc(copy, copysize);
		if (newcopy == NULL) {
			copysize = 0;
			free(copy);
			copy = NULL;
			return (NULL);
		}
		copy = newcopy;
	}
	(void) memmove(copy, str, len - 3);
	copy[len - 3] = 'l';
	copy[len - 2] = ch;
	copy[len - 1] = '\0';
	return (copy);	
}
```

Please translate translate Part 2 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
