#include <stdio.h>
#include <string.h>
#include <errno.h>

int main(void)
{
    int err = EILSEQ; /* пример номера ошибки */
    char buf[256];

#if defined(_WIN32) || defined(_MSC_VER)
    /* На Windows/MSVC используем strerror_s */
    errno_t er = strerror_s(buf, sizeof buf, err);
    if (er == 0)
        printf("Error %d: %s\n", err, buf);
    else
        fprintf(stderr, "strerror_s failed: %d\n", (int)er);
#else
    /* POSIX: strerror_r — два варианта интерфейса.
       Для glibc с _GNU_SOURCE возвращается char*, иначе int. */
#if defined(__GLIBC__) && defined(_GNU_SOURCE)
    char *msg = strerror_r(err, buf, sizeof buf);
    printf("Error %d: %s\n", err, msg);
#else
    int rc = strerror_r(err, buf, sizeof buf);
    if (rc == 0)
        printf("Error %d: %s\n", err, buf);
    else
        fprintf(stderr, "strerror_r failed: %s\n", strerror(rc));
#endif
#endif

    return 0;
}