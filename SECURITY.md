# Security Policy

## Reporting a vulnerability

Please **do not** open a public GitHub Issue for security vulnerabilities.

Email: security@privacychase.com  
Expected response time: 48 hours

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Your suggested fix if you have one

We will acknowledge receipt, investigate, and coordinate a fix and disclosure timeline with you.

## Scope

Things we care about most:
- Any finding where Auris makes an unexpected network call (this violates our core privacy charter)
- Privilege escalation via the audio driver
- Arbitrary code execution via malformed headphone profile files or ONNX models
- Local data exfiltration

## Supported versions

Only the latest release is actively supported.
