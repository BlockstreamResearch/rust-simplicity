#ifndef WRAPPER_H
#define WRAPPER_H

#define WRAP_(jet) \
bool rustsimplicity_0_5_c_##jet(frameItem* dst, const frameItem* src, const txEnv* env) { \
  bool result = rustsimplicity_0_5_##jet(dst, *src, env); \
  rustsimplicity_0_5_assert(!result || 0 == dst->offset); \
  return result; \
}

#endif
