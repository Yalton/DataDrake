export function decodeBase64(base64String: string | null): string {
    return base64String ? atob(base64String) : "error"

}

export function encodeBase64(plainString: string | null): string {
    // return btoa(plainString);
    return plainString ? btoa(plainString) : "error"
}