#ifndef WRAPPER_H
#define WRAPPER_H

#define WRAP_(jet) \
bool c_##jet(frameItem* dst, const frameItem* src, const txEnv* env) { \
  bool result = jet(dst, *src, env); \
  simplicity_assert(!result || 0 == dst->offset); \
  return result; \
}

#endif
