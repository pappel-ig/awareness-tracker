export const surveyMock = {
  age: "26-45",
  meta: {
    ip: {
      ip: "0.0.0.0",
      asn: 123,
      isp: "wilhelm.tel GmbH",
      city: "Hamburg",
      country: "Germany",
      country_code: "DE"
    },
    leak: ["Passwords"],
    tracker: [
      {
        headers: {
          mock: "mock",
        },
        created_at: "2026-10-03T17:52:16.342690+00:00",
        remote_addr: "0.0.0.0",
        remote_addr_info: {
          ip: "0.0.0.0",
          asn: 123,
          isp: "Mock",
          city: "Mock",
          country: "Mock",
          country_code: "Mock"
        }
      }
    ]
  },
  ipKnowledge: true,
  itKnowledge: 3,
  ipScareFactor: 2,
  leakKnowledge: true,
  vpn: false,
  updates: true,
  passkeys: true,
  adBlocker: true,
  twoFactor: false,
  publicWifi: false,
  passwordReuse: true,
  passwordChange: false,
  passwordManager: false,
  leakScareFactor: 4,
  securityAwareness: 4,
  fingerprintKnowledge: true,
  fingerprintScareFactor: 2,
  trackingPixelKnowledge: false,
  trackingPixelScareFactor: 4
}
