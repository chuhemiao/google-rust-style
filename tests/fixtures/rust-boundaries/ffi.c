#include <stdlib.h>
#include <stddef.h>
#include <stdatomic.h>

/* Allocation, access, and destruction stay on the calling thread. No callbacks.
 * Exactly four initialized immutable bytes; free only via fixture_free. */
static _Atomic size_t live_buffers;
unsigned char *fixture_new(void) {
    unsigned char *p = malloc(4);
    if (p != NULL) {
        for (size_t i = 0; i < 4; ++i) p[i] = (unsigned char)i;
        atomic_fetch_add(&live_buffers, 1);
    }
    return p;
}
void fixture_free(unsigned char *p) { free(p); atomic_fetch_sub(&live_buffers, 1); }
size_t fixture_live(void) { return atomic_load(&live_buffers); }
