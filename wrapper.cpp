#include <MMKV.h>
#include <vector>
#include <string>

// C ABI
extern "C" MMKV *get_mmkv_instance(MMKVMode mode, const char *cryptKey) {
    std::string *tmp = nullptr;
    if (cryptKey != nullptr) {
        tmp = new std::string (cryptKey);
    }
    auto mmkv = MMKV::defaultMMKV(mode,tmp);
    if (tmp != nullptr) {
        delete tmp;
        tmp = nullptr;
    }
    return mmkv;
}

extern "C" void init_mmkv(const char *dir, MMKVLogLevel logLevel, mmkv::LogHandler handler) {
    std::string tmp(dir);
    MMKV::initializeMMKV(tmp,logLevel,handler);
}

extern "C" void set_bool(MMKV *mmkv,bool v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,tmp);
}

extern "C" bool get_bool(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    return mmkv->getBool(tmp);
}

extern "C" void set_string(MMKV *mmkv,const char *v,const char *k) {
    std::string value(v);
    std::string key(k);
    mmkv->set(value,key);
}

extern "C" const char* get_string(MMKV *mmkv,const char *k) {
    std::string tmp(k);
    std::string result;
    mmkv->getString(tmp,result);
    char* resultCStr = new char[result.size() + 1]; 
    std::strcpy(resultCStr, result.c_str());
    return resultCStr;
}

extern "C" void enable_auto_key_expire(MMKV *mmkv,uint32_t expire) {
    mmkv->enableAutoKeyExpire(expire);
}

extern "C" void set_double(MMKV *mmkv,double v,const char *k) {
    std::string tmp(k);
    mmkv->set(v,tmp);
}

extern "C" double get_double(MMKV *mmkv,const char *k){
    std::string tmp(k);
    return mmkv->getDouble(tmp);
}

extern "C" void set_string_list(MMKV *mmkv, const char **vec, size_t length, const char *key) {
    std::vector<std::string> v(vec, vec + length);
    std::string tmp(key);
    mmkv->set(v, tmp);
}

extern "C" const char **get_string_list(MMKV *mmkv, size_t *length, const char *key) {
    std::vector<std::string> v;
    std::string tmp(key);
    mmkv->getVector(tmp, v);

    static std::vector<const char *> c_strings(v.size());
    for (size_t i = 0; i < v.size(); ++i) {
        c_strings[i] = v[i].c_str();
    }
    *length = v.size();
    return c_strings.data();
}