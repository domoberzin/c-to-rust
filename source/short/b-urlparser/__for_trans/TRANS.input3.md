
It looks good to me. Now let us look at the next part of the program:

### Part 3

```c
bool
url_is_protocol (char *str) {
  int count = sizeof(URL_SCHEMES) / sizeof(URL_SCHEMES[0]);

  for (int i = 0; i < count; ++i) {
    if (0 == strcmp(URL_SCHEMES[i], str)) {
      return true;
    }
  }

  return false;
}

bool
url_is_ssh (char *str) {
  str = strdup(str);
  if (0 == strcmp(str, "ssh") ||
      0 == strcmp(str, "git")) {
    free(str);
    return true;

  }

  return false;
}

char *
url_get_protocol (char *url) {
  char *protocol = malloc(URL_PROTOCOL_MAX_LENGTH * sizeof(char));
  if (!protocol) return NULL;
  sscanf(url, "%[^://]", protocol);
  if (url_is_protocol(protocol)) return protocol;
  return NULL;
}


char *
url_get_auth (char *url) {
  char *protocol = url_get_protocol(url);
  if (!protocol) return NULL;
  int l = (int) strlen(protocol) + 3;
  return get_part(url, "%[^@]", l);
}

char *
url_get_hostname (char *url) {
  int l = 3;
  char *protocol = url_get_protocol(url);
  char *tmp_protocol = strdup(protocol);
  char *auth = url_get_auth(url);

  if (!protocol) return NULL;
  if (auth) l += strlen(auth) + 1; // add one @ symbol
  if (auth) free(auth);

  l += (int) strlen(protocol);

  free(protocol);

  char * hostname = url_is_ssh(tmp_protocol)
           ? get_part(url, "%[^:]", l)
           : get_part(url, "%[^/]", l);
  free(tmp_protocol);
  return hostname;
}

char *
url_get_host (char *url) {
  char *host = malloc(sizeof(char));
  char *hostname = url_get_hostname(url);

  if (!host || !hostname) return NULL;

  sscanf(hostname, "%[^:]", host);

  free(hostname);

  return host;
}

char *
url_get_pathname (char *url) {
  char *path = url_get_path(url);
  char *pathname = malloc(sizeof(char));

  if (!path || !pathname) return NULL;

  strcat(pathname, "");
  sscanf(path, "%[^?]", pathname);

  free(path);

  return pathname;
}
```

Please translate translate Part 3 of the C program (provided above) to **safe** Rust. Please output code directly without explanation. Please do not skip any functions in this part. Each function should be translated completely without placeholders. DO NOT use `unsafe`.
