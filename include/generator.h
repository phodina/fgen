
#ifndef cheddar_generated_generator_h
#define cheddar_generated_generator_h


#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



Generator* generator_new(char const* project_path, char const* template_path);

void generate_file(Generator* ptr, char const* src_path, char const* dst_path);

void generator_free(Generator* ptr);



#ifdef __cplusplus
}
#endif


#endif
