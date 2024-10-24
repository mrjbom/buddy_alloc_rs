// https://github.com/spaskalev/buddy_alloc/issues/124

#define BUDDY_ALLOC_IMPLEMENTATION

#define BUDDY_PRINTF(...)

#define memset buddy_alloc_memset
#define memmove buddy_alloc_memmove

typedef long ssize_t;
typedef unsigned long size_t;

void* buddy_alloc_memset(void* dest, int val, size_t num)
{
    unsigned char* ptr = dest;
    while (num-- > 0) {
        *ptr++ = (unsigned char)val;
    }
    return dest;
}

void* buddy_alloc_memmove(void* dest, const void* src, size_t num)
{
    char* d = dest;
    const char* s = src;
    if (d < s) {
        while (num--) {
            *d++ = *s++;
        }
    }
    else {
        char* lasts = (char*)s + (num-1);
        char* lastd = (char*)d + (num-1);
        while (num--) {
            *lastd-- = *lasts--;
        }
    }
    return dest;
}

#include "buddy_alloc.h"
