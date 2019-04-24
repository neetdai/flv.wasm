export function ua_match (ua) {
    return /(edge)\/([\w.]+)/.exec(ua) ||
    /(opr)[\/]([\w.]+)/.exec(ua) ||
    /(chrome)[ \/]([\w.]+)/.exec(ua) ||
    /(iemobile)[\/]([\w.]+)/.exec(ua) ||
    /(version)(applewebkit)[ \/]([\w.]+).*(safari)[ \/]([\w.]+)/.exec(ua) ||
    /(webkit)[ \/]([\w.]+).*(version)[ \/]([\w.]+).*(safari)[ \/]([\w.]+)/.exec(ua) ||
    /(webkit)[ \/]([\w.]+)/.exec(ua) ||
    /(opera)(?:.*version|)[ \/]([\w.]+)/.exec(ua) ||
    /(msie) ([\w.]+)/.exec(ua) ||
    ua.indexOf('trident') >= 0 && /(rv)(?::| )([\w.]+)/.exec(ua) ||
    ua.indexOf('compatible') < 0 && /(firefox)[ \/]([\w.]+)/.exec(ua) ||
    []
}

export function platform_match (ua) { 
    return /(ipad)/.exec(ua) ||
    /(ipod)/.exec(ua) ||
    /(windows phone)/.exec(ua) ||
    /(iphone)/.exec(ua) ||
    /(kindle)/.exec(ua) ||
    /(android)/.exec(ua) ||
    /(windows)/.exec(ua) ||
    /(mac)/.exec(ua) ||
    /(linux)/.exec(ua) ||
    /(cros)/.exec(ua) ||
    []
}