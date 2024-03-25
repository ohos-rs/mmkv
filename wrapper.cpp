#include <MMKV.h>
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