# Authentication

---

1. Using **JWT** (JSON Web Token) for authentication over
   **HTTP Basic Auth**.

&nbsp;&nbsp;&nbsp;&nbsp;This is from [stackoverflow](https://security.stackexchange.com/a/248209/164079)!
So, what are the advantages of JWTs over HTTP Basic Auth? In no particular order:

* JWTs are generally specific to one service. If I take a JWT that tells StackExchange that I'm cbhacking and try to use
  it on GitHub, there is no chance GitHub will consider this valid. Passwords (sadly) get reused all over.
* JWTs are time-limited, and expire. If you dig through a dump of some poorly-sanitized logs from months ago and find a
  JWT I used, it will almost certainly (if the implementer had any sense) long since have expired and thus be basically
  worthless. If you instead find a basic auth string, odds are pretty good that the password it contains is still valid.
* JWTs can be issued by one service and verified by another. This is one of the standard things they're used for, in
  fact. If you log into a site A via "Sign in with Facebook" or some other SSO (Single Sign-On) provider B, the site A
  that you're logging into will never see your password; that either goes straight to the SSO provider B, or did at some
  time in the past (and your session with B is still active). Then B can return a JWT saying "The bearer of this is
  Arjun, user ID 255431" and signed in a way that A can *verify* but not *create* (this is the magic of asymmetric
  cryptography). Now site A knows who you are - so long as you send the JWT to them - without ever seeing any of your
  credentials.
* JWTs can grant scoped permissions. Suppose I have a calendar online, and since it's my calendar, I can see all the
  details of events, add and remove events, and so on. Now suppose some third-party app for scheduling calls with
  clients wants to access my calendar. I want it to know what times I'm available, but I don't want it to know what I'm
  doing during any particular slot. Maybe I'd let it add and remove those client meetings directly, but it shouldn't be
  able to remove meetings (or dentist appointments, or vacations, or friends' birthdays) that I added for my own use (
  nor should it even be able to tell what those are). With basic auth, I'd have to set up multiple unique passwords -
  one for each service (client meetings, friend birthdays, etc.) that I don't want to give full access to - or else
  share the main password and then any app that wanted to could take total control. With a JWT, each app can request
  only the permissions that it needs, and when I approve them it gets a JWT with "scope" claims that limit what the
  token can be used for.
* JWTs can be both secure and fast to verify. Password storage, necessary for basic auth, can't. Symmetric signatures (
  HMACs) on JWTs are super-fast to verify, usually at most taking a few rounds of hashing. Asymmetric signatures are
  significantly more expensive to verify, but you can still do them very quickly especially if you use efficient
  algorithms, without compromising security. Verifying passwords, however, needs to be very slow indeed, because they're
  too easy to brute-force from a leaked database if you use a fast hash or similar. Thus, password hashing is
  deliberately designed to be computationally expensive - tens of thousands of rounds of hashing is one simple way to do
  it - and is usually tuned to take somewhere between about 50ms and 250ms per user, on security-conscious sites. As
  computers get faster, you have to make the password hash even more expensive, too, which isn't a significant concern
  with JWT signatures. This means that, with basic auth, you have to either add a ton of CPU time spent doing extremely
  expensive password hashing, or you have to run a very high risk that, if your password database is leaked, people will
  be able to brute-force the hashes. With JWTs that's not nearly as big a concern (with HMAC-signed JWTs, it's basically
  not a concern at all).
* Relatedly, JWTs can be verified without a database lookup at all. Passwords, including ones sent in basic auth, can't.
  You can cache them, but that takes server-side resources and breaks down when you have a cluster of servers behind a
  load balancer. Thus, JWTs are much better for scaling to large numbers of users, where DB hits are relatively
  expensive and need to be minimized.
* Basic auth isn't really compatible with multi-factor authentication (sometimes called two-step authentication or
  similar). MFA is usually implemented with ephemeral values - one-time codes generated in an app or sent via SMS,
  challenge nonces sent to hardware tokens or push notifications, etc. - and even if there was a practical way to
  include the resulting token in the basic auth string (there isn't, for browsers), it would become invalid almost
  immediately. That's awkward if you want a session to last more than a few minutes before the user has to log in again!
  With a JWT (or any other session token), you can verify the user's credentials (including MFA) once, and then give
  them a token good for as long as you want (with JWTs in particular, it's often two tokens: the JWT itself which has a
  short lifetime, and a refresh token that lasts longer but takes a DB lookup to verify).
* A bit of a technical curiosity rather than a deliberate feature of JWT design, but: HTTP Basic Auth is automatically
  sent by the browser for any request to the relevant domain... even if the request comes from a different domain. This
  means that there's a risk of CSRF (Cross-Site Request Forgery), where if site X thinks you might be signed in to site
  Y, it can make your browser send requests to Y *and Y will think you, the user, did it*. This could be used for
  anything from posting messages on a forum to transferring money between accounts, and it would happen without your
  awareness. JWTs, by comparison, can be put in lots of different places. Cookies are common, and *used to be*
  automatically sent even on requests from other domains (just like Basic Auth), but these days there's the `SameSite`
  cookie flag that can be used to control this behavior. JWTs are also often sent in other HTTP request headers (
  often `Authorization` - the same one used for basic auth - but sometimes custom ones), and in those cases the
  requesting client must explicitly specify to send the JWT. Which means the requesting client has to know the full
  contents of the JWT, because your browser won't just automatically attach it to every relevant request.
* Logging out a user who uses a JWT is simple, just delete it from their client (this doesn't invalidate the JWT on the
  server - that's hard to do, and usually you just rely on the JWT having a short lifetime and expiring soon - but it
  works fine for Alice getting up from the computer before Bob sits down to use it). It turns out that there's no
  standardized way to make a browser forget basic auth credentials. At least one browser (Internet Explorer) has a
  special API for this, but generally speaking, if you want to log out a client that uses basic auth, the only option is
  to tell the browser that their authorization is invalid (which generally makes the browser stop sending it). That's...
  messy. Obviously for non-browser clients this is not an issue.
* Relatedly, logging in with basic auth isn't really under the control of the client application (implemented in
  HTML/CSS/JS). The web browser chooses what to show (usually a pretty ugly modal credentials prompt). Other forms of
  authentication / authorization token allow the web application to create their own login pages, which might include
  features like a "forgot password?" link or SSO login options, while a basic auth site just presents the user with a
  blank page (the content won't be loaded - or even requested - until the user signs in!) and an ugly pop-up modal box.

