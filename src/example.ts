async function getResource() {
    return {
        [Symbol.asyncDispose]: async () => {
            await someAsyncFunc();
        },
    };
}

