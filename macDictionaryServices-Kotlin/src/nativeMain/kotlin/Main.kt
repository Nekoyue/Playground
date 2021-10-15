import kotlinx.cinterop.*
import platform.CoreFoundation.*
import platform.CoreServices.DCSCopyTextDefinition

fun main() {
    define("ubiquitous").also(::println)
    define("うなぎ").also(::println)
    define("AnUndefinedWord").also(::println)

}

fun define(word: String): String? {
    val result: CFStringRef? = DCSCopyTextDefinition(
        null,
        CFStringCreateWithCString(alloc = kCFAllocatorDefault, cStr = word, encoding = kCFStringEncodingUTF8),
        CFRangeMake(loc = 0, len = word.length.toLong())
    )

    if (result != null) {
        val bufferSize =
            CFStringGetLength(result) * 4 // func returns UTF-16 character counts, times 4 for non-ASCII buffer safety

        val buffer: CArrayPointer<ByteVar> = nativeHeap.allocArray(bufferSize)
        CFStringGetCString(result, buffer, bufferSize, kCFStringEncodingUTF8)

        return (buffer.toKString())
    }
    return null
}