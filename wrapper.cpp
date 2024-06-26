#include "MMKV.h"
#include <vector>
#include <string>

// C ABI
extern "C" MMKV *get_mmkv_instance(MMKVMode mode, const char *cryptKey)
{
    std::string *tmp = nullptr;
    if (cryptKey != nullptr)
    {
        tmp = new std::string(cryptKey);
    }
    auto mmkv = MMKV::defaultMMKV(mode, tmp);
    if (tmp != nullptr)
    {
        delete tmp;
        tmp = nullptr;
    }
    return mmkv;
}

extern "C" MMKV *get_mmkv_instance_with_id(const char *mmap_id, MMKVMode mode, const char *cryptKey)
{
    std::string id(mmap_id);
    std::string *tmp = nullptr;
    if (cryptKey != nullptr)
    {
        tmp = new std::string(cryptKey);
    }
    auto mmkv = MMKV::mmkvWithID(id, mode, tmp);
    if (tmp != nullptr)
    {
        delete tmp;
        tmp = nullptr;
    }
    return mmkv;
}

extern "C" void init_mmkv(const char *dir, MMKVLogLevel logLevel, mmkv::LogHandler handler)
{
    std::string tmp(dir);
    MMKV::initializeMMKV(tmp, logLevel, handler);
}

extern "C" const char *get_mmap_id(MMKV *mmkv)
{
    std::string result = mmkv->mmapID();
    char *resultCStr = new char[result.size() + 1];
    std::strcpy(resultCStr, result.c_str());
    return resultCStr;
}

extern "C" void set_bool(MMKV *mmkv, bool v, const char *k)
{
    std::string tmp(k);
    mmkv->set(v, tmp);
}

extern "C" bool get_bool(MMKV *mmkv, const char *k)
{
    std::string tmp(k);
    return mmkv->getBool(tmp);
}

extern "C" void set_string(MMKV *mmkv, const char *v, const char *k)
{
    std::string value(v);
    std::string key(k);
    mmkv->set(value, key);
}

extern "C" const char *get_string(MMKV *mmkv, const char *k)
{
    std::string tmp(k);
    std::string result;
    mmkv->getString(tmp, result);
    char *resultCStr = new char[result.size() + 1];
    std::strcpy(resultCStr, result.c_str());
    return resultCStr;
}

extern "C" void enable_auto_key_expire(MMKV *mmkv, uint32_t expire) { mmkv->enableAutoKeyExpire(expire); }

extern "C" void set_double(MMKV *mmkv, double v, const char *k)
{
    std::string tmp(k);
    mmkv->set(v, tmp);
}

extern "C" double get_double(MMKV *mmkv, const char *k)
{
    std::string tmp(k);
    return mmkv->getDouble(tmp);
}

extern "C" void set_string_list(MMKV *mmkv, const char **vec, size_t length, const char *key)
{
    std::vector<std::string> v(vec, vec + length);
    std::string tmp(key);
    mmkv->set(v, tmp);
}

extern "C" const char **get_string_list(MMKV *mmkv, size_t *length, const char *key)
{
    std::vector<std::string> v;
    std::string tmp(key);
    mmkv->getVector(tmp, v);

    static std::vector<const char *> c_strings(v.size());
    for (size_t i = 0; i < v.size(); ++i)
    {
        c_strings[i] = v[i].c_str();
    }
    *length = v.size();
    return c_strings.data();
}

extern "C" void remove_value_for_key(MMKV *mmkv, const char *key)
{
    std::string tmp(key);
    mmkv->removeValueForKey(tmp);
}

extern "C" void remove_values_for_keys(MMKV *mmkv, const char **vec, size_t length)
{
    std::vector<std::string> v(vec, vec + length);
    mmkv->removeValuesForKeys(v);
}

extern "C" bool contains_key(MMKV *mmkv, const char *key)
{
    std::string tmp(key);
    return mmkv->containsKey(tmp);
}

extern "C" const char **all_keys(MMKV *mmkv, size_t *length)
{
    auto v = mmkv->allKeys();

    static std::vector<const char *> c_strings(v.size());
    for (size_t i = 0; i < v.size(); ++i)
    {
        c_strings[i] = v[i].c_str();
    }
    *length = v.size();
    return c_strings.data();
}

extern "C" size_t get_actual_size(MMKV *mmkv) { return mmkv->actualSize(); }

extern "C" size_t get_total_size(MMKV *mmkv) { return mmkv->totalSize(); }

extern "C" size_t get_value_size(MMKV *mmkv, const char *key, bool actual)
{
    std::string tmp(key);
    return mmkv->getValueSize(tmp, actual);
}

extern "C" void clear_all(MMKV *mmkv, bool keep_space) { mmkv->clearAll(keep_space); }

extern "C" void clear_memory_cache(MMKV *mmkv, bool keep_space) { mmkv->clearMemoryCache(keep_space); }

extern "C" size_t count(MMKV *mmkv, bool filter_expire) { return mmkv->count(filter_expire); }

extern "C" size_t back_up(const char *root_dir, const char *mmap_id)
{
    std::string dir(root_dir);
    if (mmap_id == nullptr)
    {
        return MMKV::backupAllToDirectory(dir);
    }
    else
    {
        std::string id(mmap_id);
        return MMKV::backupOneToDirectory(id, dir);
    }
}

extern "C" size_t restore(const char *root_dir, const char *mmap_id)
{
    std::string dir(root_dir);
    if (mmap_id == nullptr)
    {
        return MMKV::restoreAllFromDirectory(dir);
    }
    else
    {
        std::string id(mmap_id);
        return MMKV::restoreOneFromDirectory(id, dir);
    }
}

extern "C" void remove_storage(const char *mmap_id)
{
    std::string tmp(mmap_id);
    MMKV::removeStorage(tmp);
}
