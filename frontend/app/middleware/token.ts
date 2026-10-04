export default defineNuxtRouteMiddleware(async (to, _) => {
    const apiBase = useApiBase()

    const token = to.query.token


    if (!token) {
        return navigateTo('/start')
    }

    try {
        const response = await $fetch<{ survey_sent: boolean}>("participants", {
            baseURL: apiBase,
            method: "GET",
            query: { token },
            headers: { "Content-Type": "application/json" },
        })
        if (response.survey_sent) return navigateTo('/end')
    } catch (err) {
        return navigateTo('/start')
    }
})