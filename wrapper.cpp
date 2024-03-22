#include <MMKV.h>
#include <string>

// C ABI
extern "C" MMKV *get_mmkv_instance() {
    return MMKV::defaultMMKV();
}

extern "C" void init_mmkv(const char *dir) {
    std::string tmp(dir);
    MMKV::initializeMMKV(tmp);
}

extern "C" void set_bool(MMKV *mmkv,bool v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,tmp);
}

extern "C" bool get_bool(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    return mmkv->getBool(tmp);
}

extern "C" void set_i32(MMKV *mmkv,int32_t v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,tmp);
}

extern "C" int32_t get_i32(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    return mmkv->getInt32(tmp);
}

extern "C" void set_f64(MMKV *mmkv,double v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,k);
}

extern "C" double get_f64(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    return mmkv->getDouble(tmp);
}
