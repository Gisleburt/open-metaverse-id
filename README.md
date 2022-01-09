Open Metaverse Id
=================

Open Metaverse Id (OMID) is a simple, platform independent, method of identifying people in the
metaverse.

Contents
--------

1. Abstract
3. Features 
2. Terminology
4. Explanation
   1. Root Identity
   2. Intermediate Identity
5. Example Usage
    1. Creating a Root Identity
    2. Using a Root Identity to authenticate with a Platform
    3. Using a Root Identity to authenticate with a Application directly
    4. Creating an Intermediate Identity for use on a device
    5. Validation an Intermediate Certificate with a service
    6. Intermediate identity revocation
    7. Root Identity revocation
6. Frequently Asked Questions

Abstract
--------

At the time of writing, "metaverse" is _just_ a buzzword, one that means different things to
different people. It is not something that exists, but is an idea of an interconnected future. A
common theme in these ideas though is that a user can traverse different services within the
metaverse and keep the same identity as they move from place to place. In its simplest form, this
could mean that your avatar (your representation in the metaverse) might remain the same as you move
from your virtual office to your favourite games.

Some form of Metaverse seems inevitability. Not as a new separate thing, but as an extension to
existing technologies like the Internet. There are many opportunities that the Metaverse will unlock
for its users, in particular, new ways to collaborate, create, play and trade with each other.

In order to enable this though, we must have a consistent way of identifying each other across
different services and platforms. This document proposes a method for doing this.

In order to Identify users we must satisfy the following features:
- Users can use Identities to locate each other across the metaverse
- Platforms and Services can use Identities to authenticate returning users
- A user's private information must not be revealed through their public Identity
- A user's Identity must work across multiple services to enable a consistent experience for the
  user
- A user can use their Identity across multiple devices

Luckily, we don't need to create anything new to achieve this. This can all be done using Asymmetric
Cryptography, and a new format of Cryptographic Certificates called Open Metaverse Id.

Features of OMID
----------------

1. Identities are owned and controlled by users
2. Identities are portable and can be used to find users across different platforms
3. Identities can be used to securely authenticate users
4. Additional Trust (eg, PGP/GPG signatures) is not required as a user is just a number
5. Identities _can_ function as blockchain identities, allowing ownership of NFTs without explicitly
   being tied to any specific blockchain or req

Terminology
-----------

### Participants

- **User:** A person who wishes to use an Service in the metaverse
- **Service:** A program or service in the metaverse that the user wishes to use. Example services
  are things like games or secure collaborative work spaces
- **Platform:** Platforms act as intermediaries between Users and Services. Example platforms
  might be Meta, Valve's Steam, HTC's Vive, Pico Interactive's Neo, etc.
- **Device:** A device that allows a user to participate in the Metaverse. This may be a device with
  direct access to a user's Identity, such as a phone or computer, or a device without direct access
  to a user's Identity, for example a stand-alone device such as a Quest or Pico Neo 
  
### Components
- **Identity:** A user's identity is, at its core, a asymmetric cryptographic keypair. Platforms and
  Services store the Public Key, and can confirm the users identity by challenging the private key.
  A user proves their identity answering the challenge.
- **Identity Manager:** Software the user can use to create and manage identities and prove identity
  by answering challenges. It can also be user to authorise intermediate identities that can act on
  a user's behalf.
- **Root Identity:** This is a users identity and is formed of two parts.
    - **Public Certificate:** This certificate is public and can be shared freely.
    - **Private Document:** This document must be kept private and is used to prove the identity
- **Intermediate Identity:** Its not always possible or practical to access the Root Identity. Fo
  this reason we can "authorise" a third party to act on the users behalf. Like the Root Identity
  the Intermediate identity is made up of two parts
    - **Public Certificate:** This certificate is public and can be shared freely.
    - **Private Document:** This document must be kept private and is used to prove the identity

Explanation
-----------

The method of identification and authentication uses asymmetric cryptography and signed
certificates.

An Identity is made up of two parts, a private document and a public certificate. The private key
remains inside a Identity Manager and is used to prove Identity by answering challenges from third
parties who have the Public Certificate. The Public Certificate, can be freely shared and used to
prove the Identity of a user who holds the Private Document 

The method of identifying a user is simply the Public Key.

### Root Identity

This is a users identity and is formed of two parts.

- **Public Certificate:** This certificate is public and can be shared freely. It contains:
    - the version of OMID the certificate conforms to
    - the cryptographic algorithm in use
    - the public key
    - not before - a timestamp before which the certificate is not valid, can be creation date
    - not after (optional) -  timestamp after which the certificate is not valid
    - self-signed signature - prevents changing details of the certificate like the timestamps

- **Private Document:** This document must be kept private
    - the version of OMID the certificate conforms to
    - the cryptographic algorithm in use
    - the private key
    - a private description for the user to document the purpose or use for the key

### Intermediate Identity

Its not always possible or practical to access the Root Identity. For example, the Root Identity
might exist on a mobile phone, while the user is using a stand along device, such as a Quest, which
would make it unwieldy to answer challenges to the Quest via the Root Identity. Instead, a user can
authorise a device to act on their behalf. To do this, the device creates its own keypair, and
creates a signing request for the Root Identity.

- **Public Certificate:** This certificate is public and can be shared freely. It contains:
    - the version of OMID the certificate conforms to
    - the cryptographic algorithm in use
    - the public key (this is the key to use for challenges/authorisation)
    - not before - must not be before the authorising identity "not before"
    - not after - must not be after the authorising identities "not after"
    - self-signed signature - prevents changing details of the certificate like the timestamps
    - intermediary signature - this is the signature of the intermediary. In the example above
      this would be signed with Meta's private key. Only intermediaries signed by trusted
      intermediaries should be trusted
    - authorising signature - this includes the public key for the authorising identity (this is
      the key to use for identification)

- **Private Document:** This document must be kept private
    - the version of OMID the certificate conforms to
    - the cryptographic algorithm in use
    - the private key
    - the public key of the identity this is acting as intermediary for

Example Usage
-------------
### Creating a Root Identity

### Using a Root Identity to authenticate with a Platform

### Using a Root Identity to authenticate with a Application directly

### Creating an Intermediate Identity for use on a device

### Validation an Intermediate Certificate with a service

### Intermediate identity revocation

### Root Identity revocation

Frequently Asked Questions
--------------------------

### Why does the user sign the platforms certificate and not the other way around?

Identities must be owned and controlled by Users. The platform signs the core of the document to
confirm they are the platform that minted the Intermediate Identity. Services should only trust
Intermediate Identities from platforms they trust. The user is the final authority on whether the
Intermediate Identity may act on their behalf, and they should only authorise Identities minted by
platforms they trust.
