#ifndef DEVICEUTILS_H
#define DEVICEUTILS_H

#ifdef __cplusplus
extern "C" {
#endif

#define MAX_KEY_LEN 64
typedef enum {
    STATUS_OK = 0,
    STATUS_ERR = 1,
} Status;

typedef struct {
    const char* key;
    const char* value;
    Status status;
} DeviceProp;

DeviceProp get_device_prop_struct(const char* path, const char* key);

#ifdef __cplusplus
}
#endif
#endif // DEVICEUTILS_H

