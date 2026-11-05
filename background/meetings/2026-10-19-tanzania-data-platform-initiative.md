# Tanzania Data Platform Initiative

- **Date:** 2026-10-19
- **Participants:** Andreas Wendelboe, Lujo
- **Source:** Wispr Flow meeting `725d64b8-d20d-48bb-a975-e1840b209b4f`

## Summary

Andreas Wendelboe and Lujo reviewed the Streamlit-based Bridgecap Transparency bookkeeping app and agreed to rebuild it in HTML/CSS, mobile-first, starting with a shopkeeper MVP led by Andreas Wendelboe.

### Current App Review
- App serves shopkeepers, business owners, donors and community members (women groups) with bookkeeping and stock tracking
    - Business owners can register goods and assign them to shops A and B
- Some text is hard to read (green on green, black on black); mobile buttons are small
- Payment entry (marejesho) tracks distributor balances and exports a PDF ledger
    - Bug: revisiting the page re-enters the payment (auto-increment)
- Production records form is only for a future where villages own production sites; groups currently only sell

### Tech Direction and Hosting
- Streamlit is too limited; Lujo wants to own the code using plain HTML and CSS
- Local Tanzanian hosting considered (Dudumizi, Netpoor, 'do hosting'); one VPS plan listed location USA
    - Prices seen: 2.5 M, 3.1 M
- Home server is an option (could handle about 10,000 users) but power and internet outages are a risk
- Docker suggested so the app can move between home server and hosting provider

### Users and Product Scope
- Most users are on smartphones; internet can be slow, so the app must load fast
- Lujo is likely the only person in the village with a computer
- Shopkeeper core needs: register stock, stock in (purchases), stock out (sales)
- Community groups and production site record keeping come after shopkeepers

### Tools and AI Budget
- Claude Pro costs 45,000 shillings a month, too expensive; Lujo's AI budget is 25,000 Tanzanian shilling or below
- Andreas Wendelboe demoed Claude Design for UI prototypes and Linear (free) for task tracking
- Plan to create a GitHub organization 'Arise and Shine', keeping the Streamlit app and adding a new repo

### Funding and Vision
- Both frustrated that Engineers Without Borders is more talk than action; funding hoped after November planning meeting
- Lujo expects about 14 million Tanzanian shillings monthly income once equipment is in place
- Andreas Wendelboe suggested studying in Denmark; Lujo will look at scholarships later

### Next Steps
- (Lujo) Interview up to five shopkeepers about current practices and pain points, then send observations
- (Lujo) Learn Linear and Claude and set up an AI subscription
- (Lujo) Share a production site progress update in the WhatsApp group

### Decisions Made
- Rebuild the app as a new project in HTML and CSS, dropping Streamlit
- Start with a shopkeeper MVP before adding other user types
- Design mobile-first and optimize for fast loading
- Use Linear for task tracking
- Andreas Wendelboe leads the project

## Transcript

**Andreas Wendelboe** [00:52]: Hallo.

**Lujo** [01:00]: Hello, Andreas.

**Andreas Wendelboe** [01:01]: Hello, can you hear me?

**Lujo** [01:04]: How are you doing?

**Andreas Wendelboe** [01:06]: I'm doing really good. How about you?

**Lujo** [01:11]: I'm doing well. I'm doing well.

**Andreas Wendelboe** [01:17]: Have you found out some good stuff about AI and, yeah, coding?

**Lujo** [01:27]: Uh, not yet, man, because I was a little bit busy this week installing electricity to the site of production.

**Andreas Wendelboe** [01:38]: Oh, yeah?

**Lujo** [01:40]: Even now, I was too late. I was traveling about 32 km to the site, with a motorcycle, but fortunately I'm back home. I'm back home right now.

**Andreas Wendelboe** [01:55]: Yeah.

**Lujo** [01:57]: Yeah.

**Andreas Wendelboe** [01:58]: That's far on a motorbike.

**Lujo** [02:01]: Yeah. How are you doing?

**Andreas Wendelboe** [02:12]: Uh, I'm doing good. I, uh, I saw your email.

**Lujo** [02:18]: Okay.

**Andreas Wendelboe** [02:18]: With the—yeah, I went into the website and tried it out.

**Lujo** [02:26]: Mm-hm.

**Andreas Wendelboe** [02:30]: Um, yeah. Yeah, I don't know what all the shopkeeper links do, but I assume it's a lot of, like, yeah, bookkeeping and stuff.

**Lujo** [02:56]: Yeah, the main issue is bookkeeping, tracking the stock.

**Andreas Wendelboe** [03:03]: Yeah.

**Lujo** [03:08]: Have you opened it? Can you share it?

**Andreas Wendelboe** [03:11]: Yeah, I can share it.

**Lujo** [03:13]: Yeah.

**Andreas Wendelboe** [03:22]: Here?

**Lujo** [03:24]: Mm-hm, yep. Mm-hm. Yeah. Uh-huh, exactly. That is it. That is it.

**Andreas Wendelboe** [03:44]: So some of the stuff is a little hard to read. Like, this is green on—green text with green background, and this is black with black background.

**Lujo** [03:56]: Okay. Sorry, Andreas, can you give me one minute? Because my son is preparing coffee for me. Let me give him my instructions. Sorry.

**Andreas Wendelboe** [04:06]: Yeah, no worries.

**Lujo** [04:07]: Yeah. Hello, Andreas.

**Andreas Wendelboe** [06:03]: Hey.

**Lujo** [06:04]: Oh, I'm back.

**Andreas Wendelboe** [06:11]: Do you have good coffee in Tanzania?

**Lujo** [06:14]: Yeah, exactly. Cheers, cheers.

**Andreas Wendelboe** [06:19]: Cheers.

**Lujo** [06:26]: Okay. So have you got some ideas about—

**Andreas Wendelboe** [06:37]: Yeah. Yeah, so, um, definitely one of the things I think we should do, and we also talked about it last time, was to stop using Streamlit.

**Lujo** [06:50]: Yep, exactly.

**Andreas Wendelboe** [06:52]: Yeah, because it's pretty obvious that, like, it's not built for what you're trying to do. And, like, this—there are other good ways that we can show data and graphs and stuff. I can, for example, show you something that I've been working on.

**Lujo** [07:15]: Okay.

**Andreas Wendelboe** [07:18]: Oh, wait. This is in, like, a different company, but this is just HTML and CSS.

**Lujo** [07:33]: CSS, yep.

**Andreas Wendelboe** [07:34]: And once it loads, I can show you, like, you can still make really nice-looking graphs and stuff.

**Lujo** [07:39]: Oh, mm-hm, yep.

**Andreas Wendelboe** [07:42]: Just with, like, pure HTML and CSS.

**Lujo** [07:46]: Hmm. It was very, very hard for me.

**Andreas Wendelboe** [07:54]: Yeah. Yeah, I bet.

**Lujo** [07:58]: Yeah. You know, the main issue was, uh, after these AI tools occurred in this nowadays, you know, because I remember when I was at university at 2017, the AI tools were not available at that time. But right now, I have seen different AI tools, so I was trying to, uh, to convert my idea into this one. So the first tool that I obtained was this Streamlit.

**Andreas Wendelboe** [08:41]: Yeah.

**Lujo** [08:42]: Together with Python for coding. Super based, because also it has a free subscription, do you see?

**Andreas Wendelboe** [08:54]: Yep.

**Lujo** [08:54]: So this, what you see right now, is just an idea. So I hope, together with a collaboration with you, we are going to make a very powerful tool.

**Andreas Wendelboe** [09:09]: Yes.

**Lujo** [09:09]: And my desire was to use this is, like, to own our own code rather than depend on this framework like Streamlit, do you see?

**Andreas Wendelboe** [09:22]: Definitely, yes.

**Lujo** [09:22]: Because they are very limited. So the idea is in this way, so I hope you will help to improve this.

**Andreas Wendelboe** [09:32]: Yeah.

**Lujo** [09:32]: Yeah.

**Andreas Wendelboe** [09:33]: Yeah. Definitely, I really want to do that. Something I also just thought of now is Streamlit. I assume you're hosting this for free with Streamlit, right? And Super Base with Super Base. I think it would be really cool if we could find, like, a company in Tanzania that does, like, hosting. Because then you could, you know, help African tech companies instead of, you know, like, fucking American companies, you know?

**Lujo** [10:18]: Yeah, sure.

**Andreas Wendelboe** [10:20]: I don't know if it exists.

**Lujo** [10:22]: Yeah. There is a lot. There is a lot. Like, Dudumizi.

**Andreas Wendelboe** [10:30]: Yeah.

**Lujo** [10:30]: Yeah, they do hosting. Do you see the second one? I am familiar with this. They do hosting. They are good.

**Andreas Wendelboe** [10:39]: Yeah.

**Lujo** [10:41]: Also, let's go again back to the—oh, do you want to preview it?

**Andreas Wendelboe** [10:49]: No, we can look at it later. Yeah. Other stuff, yeah.

**Lujo** [10:54]: Do hosting. I am familiar with it.

**Andreas Wendelboe** [10:59]: Netpoor.

**Lujo** [11:02]: Netpoor.

**Andreas Wendelboe** [11:26]: Yeah. I mean, that would be super cool, right, to support Tanzanian tech companies.

**Lujo** [11:36]: Yeah, for sure.

**Andreas Wendelboe** [11:39]: And then the benefit is also that then the website is much closer to you, so the latency is much lower.

**Lujo** [11:51]: Yeah.

**Andreas Wendelboe** [11:53]: But—

**Lujo** [11:54]: Yeah, especially this do hosting.

**Andreas Wendelboe** [11:57]: Yeah.

**Lujo** [11:58]: I remember since 2014, it was there.

**Andreas Wendelboe** [12:04]: Oh.

**Lujo** [12:04]: So even when we were studying, when we were doing our undergraduate studies, the company was already there. So they have enough experience with hosting.

**Andreas Wendelboe** [12:21]: That's cool. Yeah.

**Lujo** [12:24]: Yeah. I see. You have seen their plans.

**Andreas Wendelboe** [12:30]: Yeah, I think usually, yeah, we don't want any WordPress because we're not using WordPress. I go for VPS, like virtual private server. It's basically like a Linux machine that you get, but it's like, instead of you getting a whole machine, you get, like, a virtual machine.

**Lujo** [12:54]: A virtual. Yeah, sure.

**Andreas Wendelboe** [12:55]: Basically, yeah.

**Lujo** [12:57]: Yeah.

**Andreas Wendelboe** [12:59]: I don't know how expensive they are. I don't know if it's in your budget.

**Lujo** [13:05]: Let's try to look at. Oh, 2.5 M, 3.1 M. And—

**Andreas Wendelboe** [13:18]: But this is weird. It says location USA.

**Lujo** [13:22]: Oh. And—

**Andreas Wendelboe** [13:26]: That's not good.

**Lujo** [13:28]: And how about, can we turn our own machine and turn it to convert it and act like a server?

**Andreas Wendelboe** [13:40]: Yeah, yeah, yeah. Yeah, if you have one. Like, I have under my sofa, I have a server that I use for, like, hobby projects and stuff.

**Lujo** [13:55]: Wow.

**Andreas Wendelboe** [13:57]: But yeah, if you wanted to do that, you could do it. And, I mean, you're not going to have too many users right now. Like, a small server like that can easily handle, like, 10,000 people, I would say.

**Lujo** [14:15]: 10,000?

**Andreas Wendelboe** [14:17]: Yeah.

**Lujo** [14:19]: Yeah, but for startup, it is enough. 10,000.

**Andreas Wendelboe** [14:25]: Yeah, yeah, yeah. So, but the main issue, I would say, is to make sure that it always has power and is always connected to the internet. And if there are blackouts, then it's no good because then your website is also down.

**Lujo** [14:48]: Yeah. But for our project, huh?

**Andreas Wendelboe** [14:53]: Yeah. I mean, if you—what I would recommend you do is if you use Docker. Do you know Docker?

**Lujo** [15:01]: Docker?

**Andreas Wendelboe** [15:02]: Yeah.

**Lujo** [15:03]: Okay.

**Andreas Wendelboe** [15:05]: Have you heard of Docker?

**Lujo** [15:07]: No, not yet.

**Andreas Wendelboe** [15:09]: Okay. I'll make sure to send it to you later. But what Docker does is, it basically, you can take your code and make it into what's called an image. And then that image you can run on any machine, Windows, Mac, Linux, whatever. It doesn't care what machine it's on because the container is like its own little—you could almost think of it like a virtual machine, but it's even smaller and even better than a virtual machine. And then if you want to, you can run that container on your own server at home, or you could put it up to do hosting as a container there, and then they could run that. So it's very flexible. You can run it anywhere. And then if, let's say, you ever want to change that you're not happy with your home server, then you can move it over here, or if this becomes too expensive, then you can move it back to your home server.

**Lujo** [16:27]: Oh.

**Andreas Wendelboe** [16:31]: Maybe I can send you a good YouTube video.

**Lujo** [16:34]: Okay.

**Andreas Wendelboe** [16:36]: I don't know, do you watch a lot of YouTube?

**Lujo** [16:43]: Yeah.

**Andreas Wendelboe** [16:44]: A tool that can package.

**Lujo** [16:45]: I use the tool.

**Andreas Wendelboe** [16:47]: Okay. I'll send you some of this good stuff.

**Lujo** [16:53]: Yeah, I have noted it, the Docker.

**Andreas Wendelboe** [16:57]: Nice.

**Lujo** [16:57]: And the main issue with the technology, it need to every day, they are keep updating, so it need also you to keep learning.

**Andreas Wendelboe** [17:11]: Yes. That's the most important part about being a software engineer is to always keep learning and always keep trying new stuff.

**Lujo** [17:28]: Yeah, every day.

**Andreas Wendelboe** [17:30]: Yeah.

**Lujo** [17:38]: Oh.

**Andreas Wendelboe** [17:50]: Okay. But yeah, it would be really cool to, like, have your own Bridgecap Transparency URL, right, and have it with the Tanzania, like.

**Lujo** [18:13]: Domain.

**Andreas Wendelboe** [18:14]: Domain. Yep. And have it hosted by a server in Tanzania. So it's like super.

**Lujo** [18:22]: Yeah, for sure.

**Andreas Wendelboe** [18:23]: Yeah. And then, I guess.

**Lujo** [18:29]: And my intention was.

**Andreas Wendelboe** [18:32]: Sorry?

**Lujo** [18:32]: Yeah, my intention was to improve because, yeah, that's why I contacted Engineers Without Border, because the organization has a lot of experts and volunteers. So I hope we are going to change this idea into a well-functioning system.

**Andreas Wendelboe** [18:58]: Definitely. Yeah. Do you know how many of the users are on mobile versus computer of Bridgecap Transparency? Is it mostly mobile?

**Lujo** [19:16]: Yeah, most of them are mobile, yeah. Mobile phones.

**Andreas Wendelboe** [19:19]: Okay.

**Lujo** [19:20]: Yeah.

**Andreas Wendelboe** [19:20]: Because then we also have to make sure that it looks good on mobile, right?

**Lujo** [19:31]: Yeah, like.

**Andreas Wendelboe** [19:34]: I mean, this is already pretty good, but the buttons are a little small. It could be a little hard to click them, but.

**Lujo** [19:47]: Yeah, for sure.

**Andreas Wendelboe** [19:48]: It's still, like, it's okay. But if you want, I have this thing. I have something called Claude Design, which is like a way to design software to make it, like, look nice. And that's something, if you want, I could try to make a design prototype for you for, like, how it could look to make it look nice and make it look, make it, like, responsive on mobile. I don't know if that's something you would be interested in.

**Lujo** [20:51]: What is the name of the software?

**Andreas Wendelboe** [20:56]: I'll write it in the chat here. Claude Design.

**Lujo** [21:03]: Claude Design, okay.

**Andreas Wendelboe** [21:05]: Yeah, if you have a Claude subscription, then it works.

**Lujo** [21:11]: Okay.

**Andreas Wendelboe** [21:13]: I can show you, I can give you a little demonstration. Okay, so remember the thing, the software that I showed you that I was working on? What I basically told Claude Design was that I'm working on this project and I'm trying to figure out how best to make the UI, user experience look. And then I gave it, like, some information about what the software is supposed to do. And then it went for, like, 20 minutes or whatever, and then it came back with this design for me.

**Lujo** [22:13]: Oh.

**Andreas Wendelboe** [22:15]: And then it built, this is not functional, it's only for show, right? It's only design.

**Lujo** [22:22]: Ah, okay.

**Andreas Wendelboe** [22:25]: But then it sort of, like, presented how it could look. And then it shows a few different pages and stuff. And then what's cool is that then I can ask it to, like, "Eh, I don't really like this part. Can you modify that?" And then it will get back to me and, like, switch it up.

**Lujo** [22:57]: Yeah, and the good thing with AI, normally, I really appreciate their design.

**Andreas Wendelboe** [23:05]: Yeah. They're much better at design than I am. Like, it's really good at making something look nice. But yeah, I could make a design for you and send it to you and see if you like it and if there are any things you want to change.

**Lujo** [23:37]: Also, the main issue is, do you see this, the theme that is used in this Bridgecap?

**Andreas Wendelboe** [23:45]: Yeah.

**Lujo** [23:48]: For example, I told AI that I need the themes that will reflect the project that we are doing, like, because we are transforming agricultural by-products into hygiene products. So the AI told me that when we put, for example, do you see the yellow and the green, the mixture of yellow and the green?

**Andreas Wendelboe** [24:15]: Yeah.

**Lujo** [24:16]: To reflect the sunflower and the green is for plants, do you see? That's why this color, yeah. So the main issue is an idea.

**Andreas Wendelboe** [24:30]: Yeah. Do you like the color or would you like something else?

**Lujo** [24:37]: No, but this color, it seems like it's so deep. It's so deep, so. But we need the cool, the cool color. But also that will reflect what we are doing.

**Andreas Wendelboe** [24:52]: Yeah.

**Lujo** [24:53]: Because also my passion is to link all projects that we are doing is, like, with one consistent idea.

**Andreas Wendelboe** [25:07]: Yes.

**Lujo** [25:08]: Yeah.

**Andreas Wendelboe** [25:12]: And you say all projects that you're doing, what do you mean by that? Is Bridgecap Transparency one of the projects or does it include many projects?

**Lujo** [25:26]: Yeah, it includes many projects. For example, Bridgecap Transparency and Bridgecap Transparency right now, we are giving digital tools to this small farm holder, do you see?

**Andreas Wendelboe** [25:41]: Yeah.

**Lujo** [25:44]: How they can keep the record. For example, those groups of women also, they can keep the record of what they are doing, these community activities, do you see? Also for those shopkeepers.

**Andreas Wendelboe** [26:01]: Yeah.

**Lujo** [26:01]: Because also they are farmers, do you see? Yeah.

**Andreas Wendelboe** [26:06]: Yes. Is that what these pages are? You have shopkeeper, you have business owner, you have donor, and community member?

**Lujo** [26:15]: Community member, this we are talking about those women groups, do you see?

**Andreas Wendelboe** [26:23]: Yeah, yeah.

**Lujo** [26:24]: For example, group A is doing a soap production. So how they can keep records of what they are doing, the system will help them. Also we have shopkeepers. For example, you, Andreas, you own maybe two shops, do you see?

**Andreas Wendelboe** [26:46]: Yeah.

**Lujo** [26:46]: So you are a business owner of shop A and shop B. So you need to track how stock moves, how the income, how the revenue, do you see?

**Andreas Wendelboe** [27:00]: Yeah.

**Lujo** [27:00]: For example, the link that I sent to you.

**Andreas Wendelboe** [27:07]: Yeah.

**Lujo** [27:09]: Yeah, the link that I sent to you, the username and password, that one is one of the shops that is owned by the use of this system. And he agreed that, for example, he owned the shop, what do we call? We call it the motorcycle spare parts, do you see?

**Andreas Wendelboe** [27:36]: Yeah.

**Lujo** [27:37]: Also he has a shop, a special shop for selling this stuff like sugar, salt, flours, and other basic needs. So in this system, we call him as a business owner.

**Andreas Wendelboe** [27:59]: Okay.

**Lujo** [27:59]: So have you got an idea what I'm talking about?

**Andreas Wendelboe** [28:04]: Yeah, I do.

**Lujo** [28:04]: So how business owner will manage shop A and shop B?

**Andreas Wendelboe** [28:10]: Yes.

**Lujo** [28:12]: Yeah.

**Andreas Wendelboe** [28:13]: And so it sounds also like this is not only soap selling people, right? It's many different types of shop. It's motorcycle spare parts and okay.

**Lujo** [28:32]: Other shops, yeah. For example, clothes, because we have a lot of entrepreneurs here in Tanzania.

**Andreas Wendelboe** [28:41]: Yeah.

**Lujo** [28:41]: None of them, they own shops, do you see?

**Andreas Wendelboe** [28:44]: Yes.

**Lujo** [28:45]: Yeah. And they need software.

**Andreas Wendelboe** [28:49]: They need software.

**Lujo** [28:49]: Yeah. And we don't have software in Tanzania, but to the big cities like Dar es Salaam, Arusha, there are these developers. But me, according to my passion, according to my work that I'm doing, I'm really based in these rural areas.

**Andreas Wendelboe** [29:09]: Yes.

**Lujo** [29:10]: So, and I hope when we are building a good thing, everyone will want the solution. The main issue is the solution, yeah.

**Andreas Wendelboe** [29:21]: Yes.

**Lujo** [29:22]: Also, we do have farmers, do you see? The farmers.

**Andreas Wendelboe** [29:27]: Yeah.

**Lujo** [29:27]: For example, nowadays in Tanzania, there is this project called Building the Better Tomorrow, do you see?

**Andreas Wendelboe** [29:37]: Yeah.

**Lujo** [29:39]: This project was initiated by the Ministry of Agriculture. So a lot of young men are now engaging with agricultural activities, do you see?

**Andreas Wendelboe** [29:52]: Yeah.

**Lujo** [29:54]: Yeah, so the system will help, yeah, the system will help the user, do you see, how to keep the records of agribusiness.

**Andreas Wendelboe** [30:09]: Yeah.

**Lujo** [30:09]: So, I don't know to the technical issue, but it is a big project. But what I'm thinking is to make a multipurpose system, yeah.

**Andreas Wendelboe** [30:25]: Yes. Yeah, and is it a multipurpose like bookkeeping system or is it just?

**Lujo** [30:45]: It's like mainly as a bookkeeping. For example, one of the users, let's say he is from Building the Better Tomorrow, for example.

**Andreas Wendelboe** [31:00]: Yes.

**Lujo** [31:00]: Even there is a lot of youth in rural areas, they are doing smart agriculture. So the system will help to keep track. When I started my agribusiness activity, what I accomplished in year one. So that records will help. So with this system, we'll help him or her to, yeah, to do better in the next year. Because without record, you know, you can't move because you don't have a right decision.

**Andreas Wendelboe** [31:41]: Yes.

**Lujo** [31:42]: But if, for example, in year one, I harvest maybe 100 bags of maize, you see? And I sold in this price, what, after removing my expenses, what profit I gained, this one, do you see? Something like that. But it is a wide project. That's why I said, "Biashara yako mkononi mwako," that means that your business is in your hand.

**Andreas Wendelboe** [32:18]: Yeah.

**Lujo** [32:19]: Your business is in your hand. "Biashara yako mkononi mwako." So it depends what business are you doing. Because I remember in one class of social entrepreneur studies that I did online, I got a very good definition about business. When you define business, we say that business is about needs fulfilling and problem solving.

**Andreas Wendelboe** [32:57]: Yes.

**Lujo** [32:57]: Do you see? That is a business. It's about need fulfilling and it is about problem solving.

**Andreas Wendelboe** [33:07]: Yeah.

**Lujo** [33:08]: For example, a young man from university, he decided to, like me, I decided to come back to the community. So what I'm going to do is solving problem. While solving problem is what I'm doing the business, do you see?

**Andreas Wendelboe** [33:28]: Yeah.

**Lujo** [33:29]: Yeah. It is something like that. So that's why I said, "Biashara yako mkononi mwako," your business is in your head.

**Andreas Wendelboe** [33:38]: Yeah.

**Lujo** [33:38]: Yeah.

**Andreas Wendelboe** [33:39]: Makes good sense.

**Lujo** [33:41]: Yeah. So I don't know what is your idea? What can you say about this?

**Andreas Wendelboe** [33:52]: I mean, how can I see the other pages, like the business owner or whatever?

**Lujo** [34:05]: Ah, okay. You need to log in as a business owner.

**Andreas Wendelboe** [34:10]: Yeah.

**Lujo** [34:10]: Then you will see all the functionality about business owner.

**Andreas Wendelboe** [34:15]: Yeah.

**Lujo** [34:17]: Okay, what you can do, but I didn't, go to the register first.

**Andreas Wendelboe** [34:24]: Yeah.

**Lujo** [34:25]: When you register, let's go. Then you will register as a business owner and all those functionality about business owner will come.

**Andreas Wendelboe** [34:40]: Yeah. That's pretty cool. Is it free?

**Lujo** [34:58]: Yeah, it is free. It is free.

**Andreas Wendelboe** [35:00]: Okay. Should I fill it in or?

**Lujo** [35:13]: Yeah, just fill all of them because it will require to be filled. Just random number, not exactly one.

**Andreas Wendelboe** [35:39]: Okay, so here it would also be nice if you were automatically logged in after you registered, you know?

**Lujo** [35:50]: Yeah, yeah, yeah. Also, for example, someone has registered somewhere, it needs also to get confirmation from system administrator, do you see? Something like that.

**Andreas Wendelboe** [36:03]: Yeah.

**Lujo** [36:04]: Yeah, do you see the so the dashboard of business owner, it will be different as a shopkeeper.

**Andreas Wendelboe** [36:17]: Yes.

**Lujo** [36:18]: Do you see this functionality? For example, business owner can register stuff, I mean, with the goods, can register goods, what he used to. Also, he can assign, I will assign this one to the shop A maybe.

**Andreas Wendelboe** [36:41]: Yeah.

**Lujo** [36:41]: Yeah.

**Andreas Wendelboe** [36:47]: This is cool.

**Lujo** [36:50]: So the dashboard of business owner, it will be different from a shopkeeper.

**Andreas Wendelboe** [36:55]: Yeah.

**Lujo** [36:57]: Okay, let's go to the let me try. Let's go to the community member. I will show how community member look like.

**Andreas Wendelboe** [37:12]: Yeah. You want to share your screen?

**Lujo** [37:16]: No, just open it. Go to the login. I will share.

**Andreas Wendelboe** [37:21]: Okay.

**Lujo** [37:22]: I will share you the login credentials. For example, I registered right email, Joice John Tizzy. Let me write the comment here.

**Andreas Wendelboe** [37:51]: Yeah.

**Lujo** [37:55]: Where is the comment?

**Andreas Wendelboe** [38:01]: It's over on the right, yeah.

**Lujo** [38:03]: Yeah. Copy this one. Joice John Tizzy 5 at gmail.com. That is a username.

**Andreas Wendelboe** [38:35]: Yeah.

**Lujo** [38:37]: Then password, password is at dan.

**Andreas Wendelboe** [39:03]: Are these real accounts?

**Lujo** [39:06]: Yeah, for example, this one I registered for my family.

**Andreas Wendelboe** [39:10]: Yeah.

**Lujo** [39:11]: But just a demo. For example, say, "Welcome, Joice John, Quintini family." So we tried to put some datas, but also we paused it because we directed our effort to other activities like I was waiting for this laptop also.

**Andreas Wendelboe** [39:38]: Yeah.

**Lujo** [39:39]: Yeah. So do you see the community? This it will be the dashboard. Because we are doing the community activities, I registered as a community member. For example, yeah, are you there to the sidebar?

**Andreas Wendelboe** [40:05]: Yeah.

**Lujo** [40:08]: Sidebar, right?

**Andreas Wendelboe** [40:09]: Yeah.

**Lujo** [40:11]: Yeah, it's saying, for example, it has a learning material. Do you see?

**Andreas Wendelboe** [40:16]: Yes.

**Lujo** [40:17]: For example, we are putting try to open it. So we will put all learning material like how to make soap, certain skills like how to make bar soaps.

**Andreas Wendelboe** [40:32]: Yeah.

**Lujo** [40:34]: So the registered member will have access to learning material. Then the third function, it is sajili wakala. Sajili wakala is, for example, me, let's say, after we train someone how to make a soap, do you see? And that woman want to open a small soap-making production site.

**Andreas Wendelboe** [41:06]: Yeah.

**Lujo** [41:07]: Also, it has a distributor for soaps, do you see? Then he or she will be able to register an agent, the distributor. But in Swahili, we say wakala, or so.

**Andreas Wendelboe** [41:24]: Okay, yeah.

**Lujo** [41:26]: So that he or she will keep tracking of it. Then it has record ya matumizi, the fourth one. Record ya matumizi, it is about expenditure. What expenditure are you doing?

**Andreas Wendelboe** [41:47]: Yes.

**Lujo** [41:48]: New activities. So you will, do you see how about stocks, electricity, salaries, stuff like that? Then he or she will be able to record the expenditure. Also, when you go to the report of matumizi, for example, here, the second button, yeah, that one is showing the expenditure that you have done. For example, right now, it say, "Hakuna matumizi." It say that there is no expenditure that but when you register, it will appear here. Do you see?

**Andreas Wendelboe** [42:36]: Yes.

**Lujo** [42:38]: Then let's go quick to the another function is about the fifth one is about taarifa za uzalishaji. Taarifa za uzalishaji, this is Swahili. Uzalishaji means the records of production.

**Andreas Wendelboe** [43:01]: Okay.

**Lujo** [43:02]: So you will keep the records of your production that you did. For example, when you click it, it will come, the form will come. Where have you done your production? For example, in my home, Dongobesh, what I produce, for example, is liquid soaps. You will record it. So it is something like that.

**Andreas Wendelboe** [43:39]: Do your employees use this? Like in your shed or your factory or?

**Lujo** [43:55]: Yeah, this is what we was planning for them to start using it. But right now, we are still in early stages. For example, we need to develop this system to be very strong.

**Andreas Wendelboe** [44:12]: Yeah.

**Lujo** [44:14]: Also, it need also to give them the training, do you see? How to use this. But this was just a demo for me and my family.

**Andreas Wendelboe** [44:26]: Yeah.

**Lujo** [44:27]: So every agent or ambassador of product or group, the women and the youth group, they will use this system in doing something like this, yeah.

**Andreas Wendelboe** [44:41]: Yeah, but they won't be producing soap, right? They'll be selling soap, the groups.

**Lujo** [44:48]: Yep, yep. For example, group. For example, let's take an example of maybe group A has produced about maybe 500 soap bar, do you see?

**Andreas Wendelboe** [45:10]: No, because I thought groups didn't produce soap. I thought your employees, like, didn't you say?

**Lujo** [45:23]: Yeah, we don't have an employee 100%. We call them the group leaders, do you see?

**Andreas Wendelboe** [45:35]: Yeah.

**Lujo** [45:36]: They come to the production site to produce soap.

**Andreas Wendelboe** [45:44]: I'm confused because I thought you said that that is not the plan anymore, that the plan going forward is that you would have your own employees, like seven or eight people doing the producing, and then these youth and women groups that they would be selling. Or have I misunderstood something?

**Lujo** [46:14]: Okay, okay, okay. Are you talking about the, for example, the site that we are on right now, do you see? Yeah, we do have five staff right now. So what they're doing, they help other groups to do their production. After we do production, then we distribute to the groups according to the request that group have requested to the production site, yeah.

**Andreas Wendelboe** [46:54]: Yeah.

**Lujo** [46:55]: And what group doing is distribution of those soaps or liquid soaps to their villages.

**Andreas Wendelboe** [47:05]: Yeah, they're only selling, right?

**Lujo** [47:07]: They are only selling, yeah.

**Andreas Wendelboe** [47:09]: Okay, so why do they need this form for how much production, how much liquid soap, and where they produce it? Why do they need that if they're not producing any?

**Lujo** [47:25]: This was the assumption here was used for maybe one day village A will own their production site.

**Andreas Wendelboe** [47:37]: Okay. Yes.

**Lujo** [47:40]: For future, but right now, they don't need this one.

**Andreas Wendelboe** [47:44]: Okay, yes.

**Lujo** [47:46]: Yeah.

**Andreas Wendelboe** [47:48]: Now I understand.

**Lujo** [47:50]: Okay.

**Andreas Wendelboe** [47:54]: But there's a lot of features here. There's so many things it can do and like, oh.

**Lujo** [48:02]: Yeah, for example, here, yeah. The interesting thing, do you see? For example, yeah. And this, I really appreciated this feature. For example, do you see the marejesho? Marejesho.

**Andreas Wendelboe** [48:22]: Yeah.

**Lujo** [48:25]: Yeah, marejesho. It say, "Pokeya marejesho." Pokeya marejesho, it is like payment entry. For example, when maybe let's say group A requested about 500 soap bar, do you see?

**Andreas Wendelboe** [48:46]: Yeah.

**Lujo** [48:48]: Then they distribute to their group member according to their agreement, do you see? Because after selling, they will pay back, do you see? For example, we do have a distributor called Eustini Quintini. Then the system will show that Eustini Quintini has taken the goods that worth about maybe 10,000 Tanzanian shilling. Then Eustini paid back maybe 2,000 Tanzanian shilling.

**Andreas Wendelboe** [49:30]: Yeah.

**Lujo** [49:31]: So the balance is 8,000. So the group leaders will be able to monitor, do you see?

**Andreas Wendelboe** [49:42]: Yeah.

**Lujo** [49:43]: Yeah, have you got the idea?

**Andreas Wendelboe** [49:47]: Yeah, I understand.

**Lujo** [49:49]: Then it will, yeah. For example, try to make maybe payment. For example, Eustini now want to pay. It show that Eustini has a balance of 1,000. If you try to make payment, for example, one, let me let's say 100.

**Andreas Wendelboe** [50:21]: Yeah.

**Lujo** [50:22]: 100. He paid in cash. Date is 1910, October 2026.

**Andreas Wendelboe** [50:31]: Yeah.

**Lujo** [50:32]: Then give description of payment maybe for soap bar. Soap bar.

**Andreas Wendelboe** [50:43]: Yeah.

**Lujo** [50:44]: Then record it. After recording.

**Andreas Wendelboe** [50:48]: Oh, yeah.

**Lujo** [50:49]: Yep. Try to, have you see?

**Andreas Wendelboe** [50:54]: Yeah.

**Lujo** [50:55]: Yeah.

**Andreas Wendelboe** [51:01]: It's cool with the balloons.

**Lujo** [51:04]: Yeah. So, yeah, the balloon show that the payment is success.

**Andreas Wendelboe** [51:14]: Yeah.

**Lujo** [51:15]: Yeah. Have you get an idea? So.

**Andreas Wendelboe** [51:19]: Yeah, yeah.

**Lujo** [51:20]: Then come back. Let's see the report. Let's go at the top. Scroll, scroll. Yeah. Report ya marejesho. So this one, it, do you see? You can select, come here. You can select your ambassador or distributor. For example, let's take Eustini. In Eustini, you will see Pakuya report in PDF. You can download it in a PDF. So it will give you the ledger, the ledger of records, of all records of payment that this guy came and take this one. He paid this one. The balance is this one. Have you see?

**Andreas Wendelboe** [52:15]: Yes.

**Lujo** [52:18]: Yeah, something like that. Then it has a logo of biashara yako mkononi mwako. Yeah. But do you see the weakness? If you will keep visiting this page, it will auto-increment. This is one of the.

**Andreas Wendelboe** [52:40]: Yeah, because.

**Lujo** [52:41]: Weakness of this.

**Andreas Wendelboe** [52:43]: I saw that it hadn't entered two times 100, but it was.

**Lujo** [52:52]: Yeah. Do you see?

**Andreas Wendelboe** [52:58]: Yeah, because here, there are two of these, but I only entered it once, right?

**Lujo** [53:14]: So that it has a code that need maybe to be stopped when you keep visiting this. Yeah.

**Andreas Wendelboe** [53:23]: Yeah.

**Lujo** [53:26]: So that is how you keep it has a ledger, special ledger that is keep tracking the records of maybe this guy has taken these goods and he made this payment. Yeah, something like that.

**Andreas Wendelboe** [53:48]: Yeah. Is this the GitHub repository?

**Lujo** [53:54]: Yeah, it is a GitHub.

**Andreas Wendelboe** [53:57]: Is it on purpose that it's public?

**Lujo** [54:03]: Yeah, I made it public. I made it public.

**Andreas Wendelboe** [54:06]: Okay.

**Lujo** [54:13]: But also, my GitHub account, it seems that it is public, right? Once I shared the link, it's opening.

**Andreas Wendelboe** [54:28]: Yeah. Yeah, this is your account.

**Lujo** [54:41]: I just made it, just made it so roughly, roughly, just to start to test the idea.

**Andreas Wendelboe** [54:50]: Yeah. What would you like my help with?

**Lujo** [55:03]: Yeah, maybe first of all, let's reconstruct it again, the system. And let's start small, but to make very well-functioning and well-good system.

**Andreas Wendelboe** [55:22]: Yeah.

**Lujo** [55:23]: Let's move from this framework, like Streamlit. I really appreciate to use HTML and CSS because just matter of coding.

**Andreas Wendelboe** [55:39]: Yeah.

**Lujo** [55:39]: And after we test maybe after somehow, after we test it, maybe one day we will host it. Do you see?

**Andreas Wendelboe** [55:52]: Yes.

**Lujo** [55:52]: But let's start with this small functionality and these small features. Let's try like, okay, let's start maybe with registration issue after one. But are you agreed with, for example, to start define our user?

**Andreas Wendelboe** [56:17]: Yes.

**Lujo** [56:18]: Yeah, and let's start maybe with the shopkeepers because it has a lot of user right now. Then let's create the functionalities or features about shopkeeper.

**Andreas Wendelboe** [56:31]: Yeah.

**Lujo** [56:33]: What is your idea?

**Andreas Wendelboe** [56:35]: I really like your idea of starting new and starting small with focusing on one thing and then making that really good and work really well before we start adding more functionality. Because the issue is it can quickly become very complicated and way too much.

**Lujo** [57:03]: Sure.

**Andreas Wendelboe** [57:04]: So that's why I like your idea of starting new and then starting with the most important functionality. And if that is shopkeeper, if shopkeepers are the ones that use this the most, then I would start with shopkeeper. If it's something else, then let me know. But shopkeeper sounds like a good place to start. And then we try to figure out what do shopkeepers need? What is their most important thing for them? And then we build this thing that I like to call a MVP, a Minimal Viable Product. It is basically like what is the smallest product, the simplest that we could build that is actually useful for someone?

**Lujo** [58:05]: Yeah.

**Andreas Wendelboe** [58:06]: And of course, it's not a huge bookkeeping software. It's just very small. What's the minimal amount of features that a shopkeeper would need for them to be able to use this and for them to be able to it be useful? And then we build that.

**Lujo** [58:27]: Yeah, I appreciate it. Yeah, so after finishing maybe shopkeeper, then we will start this community member or community groups. For example, right now, our production site, it will start operation. Yeah, so we need a smart record keeping. For example, that template that Kristoffer sent us, I think it will be better for us to help them, to help somebody like Kristoffer, to help our production site to start recording, something like that. So it will really, really help in our future.

**Andreas Wendelboe** [59:13]: And the production site recording, do you think that will be with a phone or with a computer? Will they have access to a computer or a phone at the production site?

**Lujo** [59:29]: Yeah, I can say both of them, but normally is a phone, phone.

**Andreas Wendelboe** [59:36]: Okay.

**Lujo** [59:37]: Because even me, I can't carry my laptop or just phone, it will be very quick.

**Andreas Wendelboe** [59:43]: Yeah. How many people have a computer? Percentage would you guess? In your village and stuff, is it?

**Lujo** [59:54]: Oh, I can say in all village, only one, me.

**Andreas Wendelboe** [59:59]: Oh, wow. You're a trendsetter.

**Lujo** [60:04]: I was thinking that to buy one desktop computer in our production site. So those staff who maybe train them how to use it and to enter this, how to use this form and record data entry, something like that. That is for future. But right now, many of them, they have a smartphone.

**Andreas Wendelboe** [60:33]: Yeah.

**Lujo** [60:34]: Yeah.

**Andreas Wendelboe** [60:35]: Because then I think we should build something that is amazing for a phone, like a small screen, but also works for a computer. But it should be phone first. That's the most important thing is the phone.

**Lujo** [60:53]: Yeah, for sure. Yeah.

**Andreas Wendelboe** [60:55]: And it should load really fast because I assume, correct me if I'm wrong, but I assume that internet might not be the quickest everywhere, right?

**Lujo** [61:07]: Yeah. Yeah, sure. Sure. That is real. Yeah.

**Andreas Wendelboe** [61:11]: Yeah.

**Lujo** [61:14]: And for example, even this guy who are using this software right now, the shopkeeper, but he has a laptop. He's a government officer.

**Andreas Wendelboe** [61:27]: Yeah.

**Lujo** [61:28]: He's a community development officer in Bashe village. Fortunately, he has a laptop, but normally prefer smartphone for data entry. Yeah.

**Andreas Wendelboe** [61:43]: Yes. But yeah, then I think we should try to understand what do shopkeepers what's most important for shopkeepers and then build that for them first.

**Lujo** [62:06]: Maybe what we can do right now is after we agreed that maybe let's start with the shopkeeper features. So maybe I will send my input maybe to the shopkeeper right now. The important part is to record or to register stocks, something like that. Then after register stock, maybe there is this issue called stock in. Stock in. Maybe after he made purchase, then he stock in. So the system will auto. After stock in, then stock out.

**Andreas Wendelboe** [62:52]: Yes.

**Lujo** [62:52]: Stock out means selling.

**Andreas Wendelboe** [62:55]: Yeah.

**Lujo** [62:55]: Do you see?

**Andreas Wendelboe** [62:56]: Yeah.

**Lujo** [62:57]: And the good thing is the code is already here, the Python code. But because you are familiar with this AI, also maybe you will try it. But the important issue is to change from this Streamlit to the HTML and CSS.

**Andreas Wendelboe** [63:21]: Yeah.

**Lujo** [63:22]: Yeah. Yeah. And also, yeah, something like that. So we can do we can start that and work on that.

**Andreas Wendelboe** [63:33]: Yes.

**Lujo** [63:34]: And another issue is I don't know if it is the right time. I will really appreciate if I don't know if you can share maybe something like a tutorial, how can I turn this laptop or how can I subscribe to this AI tool so also I can be familiar with this AI. So also I can help you maybe after we agreed that this week we are going to do this and this one. Hello, Lucian. Can you help me with this one? Can you do this task? Something like that.

**Andreas Wendelboe** [64:17]: Yeah.

**Lujo** [64:17]: What do you think?

**Andreas Wendelboe** [64:19]: Yeah, I can help you set it up. Are you paying for any AI right now?

**Lujo** [64:31]: Right now, no. No.

**Andreas Wendelboe** [64:33]: No. Okay. I mean, I would recommend Claude. If you wanted to get an account, you can get the Claude Pro for how many shillings is that? 45,000 shillings a month. Is that too much?

**Lujo** [65:23]: 40? 45.

**Andreas Wendelboe** [65:24]: 45,000? Is that too much? Because maybe there are some better ones for cheaper.

**Lujo** [65:41]: This one is just for month, right?

**Andreas Wendelboe** [65:44]: Yeah, that's every single month.

**Lujo** [65:48]: It is a little bit.

**Andreas Wendelboe** [65:50]: Yeah.

**Lujo** [65:51]: It is a little bit expensive right now.

**Andreas Wendelboe** [65:55]: Yeah. I can look up, find something that is cheaper. What would you say is your budget for AI per month?

**Lujo** [66:15]: Maybe 25 or below.

**Andreas Wendelboe** [66:18]: Okay.

**Lujo** [66:19]: 25,000 Tanzanian shilling.

**Andreas Wendelboe** [66:22]: Yes.

**Lujo** [66:23]: I can afford that.

**Andreas Wendelboe** [66:25]: Yeah. Then I can figure out a solution and I can make you a small guide for how to set that up on your laptop.

**Lujo** [66:36]: Yep.

**Andreas Wendelboe** [66:37]: Okay. And then you mentioned how do we keep track of to-do and tasks and stuff, right?

**Lujo** [66:48]: Yep.

**Andreas Wendelboe** [66:50]: I have a or I use this software called Linear. And it is absolutely amazing for tracking tasks. It's like a planning tool. And what you do is you basically you write a task or an issue. You say like, "Switch from Streamlit to HTML, CSS." And then you write some short description. And then you can say, "Okay, who does this? Andreas or Lujo?"

**Lujo** [67:38]: Yep.

**Andreas Wendelboe** [67:39]: And then you create the issue. And then once you then you can prioritize it and you can see all your different issues. Here is a list of all the issues that I have for this project that I'm working on. And then I can just then I can just go and say like, "Okay, I'm going to work on this task now." And then I can set the status. I can set it to in progress or once I'm finished with it, I can set it to done. And then I know I keep track of what I need to do. And then I can tag people and say, "Hey, Lujo, how should I do this?" or whatever. Or you can tag me and like, "Hey, how should I do this?" And it works really well.

**Lujo** [68:48]: Yeah, it is very cool.

**Andreas Wendelboe** [68:49]: And it's free.

**Lujo** [68:50]: Oh, okay.

**Andreas Wendelboe** [68:51]: It's free. So I could.

**Lujo** [68:53]: It's cool.

**Andreas Wendelboe** [68:53]: Yeah.

**Lujo** [68:55]: I think it is called Linear.

**Andreas Wendelboe** [68:59]: Yeah. I'll text it in here.

**Lujo** [69:04]: Okay. Linear. I don't know what the normal website is. Is it called? Okay, it's just linear.app. Linear.app. I really like that. It is very helpful. It is very helpful.

**Andreas Wendelboe** [69:24]: Yes. So I could.

**Lujo** [69:27]: I think.

**Andreas Wendelboe** [69:29]: Yeah?

**Lujo** [69:29]: I think we are going to use this one.

**Andreas Wendelboe** [69:32]: Okay.

**Lujo** [69:33]: Even when you try to demonstrate it, it is very understandable.

**Andreas Wendelboe** [69:39]: Yes. Linear is one of the best software tools that exists. It is so well-made and it looks so good and it's so simple. So it's also something you could be inspired by for bridge gap transparency, to make it look and feel as simple and as good as Linear.

**Lujo** [70:07]: Okay.

**Andreas Wendelboe** [70:09]: Yeah. So I can make a Linear organization and invite you to it.

**Lujo** [70:17]: Yep.

**Andreas Wendelboe** [70:19]: And something else I was thinking is that right now you have this repository, right?

**Lujo** [70:33]: Yep. Yeah. I think.

**Andreas Wendelboe** [70:39]: In GitHub, you can make an organization, which is basically like a company. And the company can have many repositories and it can have many employees and stuff. So for example, the project that I showed you that I was working on, the company is called McDonald Pelts. And then it has the code that I was working on. So it's not tied to you as to your GitHub account. It's tied to your company instead. And then you can have multiple then you can have the Streamlit app. You can leave it here. And then we can start working on the new website. So that's also something to consider.

**Lujo** [71:45]: Yep.

**Andreas Wendelboe** [71:52]: And then you could invite me to the organization so I could work on yeah. So we work on it together.

**Lujo** [72:05]: Okay. So are you saying that in this account, we can make another repository, right?

**Andreas Wendelboe** [72:15]: Yeah. Yeah. So we can make we make an organization called Arise and Shine. And then we leave the Streamlit app as it is. And then we make a new GitHub repository also in Arise and Shine organization. And that's the new one that we're talking about.

**Lujo** [72:48]: Oh, okay.

**Andreas Wendelboe** [72:54]: Maybe I could also do that for you and then invite you as an owner. I don't know.

**Lujo** [73:01]: Yep. Yep. Yeah. Maybe you can do that because I'm not well familiar with it.

**Andreas Wendelboe** [73:07]: Yes. Yes. I will do that. And then I'll set up Linear and then I'll make a design prototype for how it could look.

**Lujo** [73:21]: Yep.

**Andreas Wendelboe** [73:24]: And then I'll send it all to you, including the guide for low-cost AI.

**Lujo** [73:35]: Okay. Yeah. Well, I appreciate it.

**Andreas Wendelboe** [73:45]: Yeah. And then if you could talk to a few different shopkeepers and understand what is it they what is it they want to do and understand also what do they do right now and what is not working for them. Because that's also what you said about being a business owner. It's about solving problems. And what was the other one? Addressing a need. Wasn't that what you said? Or

**Lujo** [74:35]: Sorry, the network was low.

**Andreas Wendelboe** [74:38]: Yeah. What was it you said about being an entrepreneur? You said it was about solving problems and something else, right?

**Lujo** [74:52]: Needs fulfilling.

**Andreas Wendelboe** [74:55]: Needs fulfilling. Yes. So what I would recommend is go talk to different shopkeepers in your village and understand their needs. What do they do right now and what doesn't work? What could be better? And understand that because we don't just want to build something that they already have. That doesn't fulfill a need. You want to build something that they don't have and is frustrating them. And yeah. And it would be good if you talked to different people and understood different problems.

**Lujo** [75:33]: Yeah, sure. Yeah, I will do that and I will send the observations.

**Andreas Wendelboe** [75:47]: Yeah.

**Lujo** [75:49]: Yeah. Yep. Oh, yeah. I see my phone needed to recharge for data again. Okay. So we agreed that you are going to make this setup, right?

**Andreas Wendelboe** [76:26]: Yeah.

**Lujo** [76:29]: Then you will invite me. Also, I will take time to learn about this issue like Linear, this issue like Cloud.

**Andreas Wendelboe** [76:46]: Oh, yeah.

**Lujo** [76:48]: Yeah. Also, I will do my best how to make this subscription so that we can start this.

**Andreas Wendelboe** [77:00]: Yeah.

**Lujo** [77:03]: Also, I will send I will talk to this maybe maximum of them. It will be five shopkeepers, what they need. Then it will be easier for us to work on their needs, right?

**Andreas Wendelboe** [77:21]: Yes.

**Lujo** [77:23]: Yeah. And maybe so my assumption is we are going to start this as a new project. Also, we will keep tracking using this Linear. So it will be a very, very good starting point.

**Andreas Wendelboe** [77:54]: Yes. And it will be structured and you can keep track of what you need to do. And yeah.

**Lujo** [78:05]: Yeah, for sure. Okay. So maybe what else maybe do you have?

**Andreas Wendelboe** [78:18]: Do you want a new logo?

**Lujo** [78:24]: Yeah. Oh, wow. Exactly. So what I can say, Andreas, what you see here is just an idea. And also what I can say is please take a lead of this project. Take a lead of this because what I can say, it is like you are fresh. You are fresh from this technology, so.

**Andreas Wendelboe** [79:03]: Yeah. Oh, now I can hear you again. What happened?

**Lujo** [79:45]: Oh, sorry. Sorry. My internet was cutting down. Also, my phone is also needed to be shut down. So what I can say, Andreas, is just take a lead of this because what I can say, you are fresh from this modern technology. So you will help a lot to direct this project. So please take a lead of this.

**Andreas Wendelboe** [80:16]: Yeah. Take a lead. Or what do you say? I can't quite hear.

**Lujo** [80:24]: Yeah. What I can say is, for example, we are a group of people. Who is a leader?

**Andreas Wendelboe** [80:32]: Ah, yes.

**Lujo** [80:33]: So yeah. That is what I mean.

**Andreas Wendelboe** [80:39]: Okay. Yeah. Yeah. I can do that.

**Lujo** [80:41]: We will keep collaborating. Maybe you can ask me, Lucian, can we modify it? What is your idea? So.

**Andreas Wendelboe** [80:49]: Yeah. Yes.

**Lujo** [80:53]: Yeah. What is your thoughts? Yeah. Is that okay?

**Andreas Wendelboe** [80:57]: Yeah. That sounds good.

**Lujo** [81:00]: Yeah. Sure.

**Andreas Wendelboe** [81:02]: And then I can also, yeah, teach you some of the stuff that I have learned working in a software company, like some of the good practices and stuff. I think you might like that.

**Lujo** [81:22]: Yeah. I will appreciate it because also you will shape me again. Yeah.

**Andreas Wendelboe** [81:31]: Yeah. But this is.

**Lujo** [81:38]: Because I have a lot I have a lot of things to do.

**Andreas Wendelboe** [81:41]: Yeah. Yeah. You're a big boss man.

**Lujo** [81:46]: Family issue. Family issue, community projects, software again.

**Andreas Wendelboe** [81:53]: Yes. You have so many projects. Yes.

**Lujo** [81:58]: Yeah.

**Andreas Wendelboe** [82:01]: Yeah. It is very impressive that you can do everything.

**Lujo** [82:11]: Maybe I can say that it's just a passion also. It is also a gift. It is a gift from God.

**Andreas Wendelboe** [82:19]: Yes.

**Lujo** [82:21]: Yeah. Even many people here in Tanzania, they are wondering. They say, "How can you afford doing this one, this one? Organize women into groups."

**Andreas Wendelboe** [82:34]: Yeah.

**Lujo** [82:35]: Yeah. It is not easy. It is not easy. But what I can say, everyone has a special gift from a creator.

**Andreas Wendelboe** [82:47]: Yes.

**Lujo** [82:49]: Yeah.

**Andreas Wendelboe** [82:50]: Yeah. But it's cool that you are doing it. It's very cool.

**Lujo** [83:01]: Yeah. To me, I normally use this principle. It is better to start with actions rather than keep talking, keep talking. It is better to start small, yeah, to express your idea. I normally believe that good people will come and will join you.

**Andreas Wendelboe** [83:27]: Yes.

**Lujo** [83:29]: Yeah.

**Andreas Wendelboe** [83:30]: Yeah. And that's also what I've been a little bit annoyed about with Engineers Without Borders is that it's a lot of talk and not enough action.

**Lujo** [83:42]: Yeah. That one yeah. Since last year, they told me maybe on February, we will keep your project in we will represent your project to the founder. But up to date, they are stealing a frequent meeting, meeting.

**Andreas Wendelboe** [84:06]: Yeah. Yeah. But I think after our meeting in November where we plan the project and stuff, and then I think you will get funding pretty soon, I hope.

**Lujo** [84:29]: Wow. That is wonderful. And you know what? The important issue, Andreas sorry, I try to keep the word Bwana Andreas.

**Andreas Wendelboe** [84:50]: What?

**Lujo** [84:52]: Sorry. When I abbreviate your name, I say I don't know if you are hearing that I am saying that Bwana Andreas. Bwana in Swahili, Bwana in English is like sir. It's like very I really appreciate your effort. That's why I say Bwana Andreas.

**Andreas Wendelboe** [85:18]: Okay.

**Lujo** [85:19]: Yeah. Bwana.

**Andreas Wendelboe** [85:21]: Thank you.

**Lujo** [85:24]: The important issue is every project when you want to turn the project into actions, it need a lot of resources, guy. And the important resources is the first one is human power. After you get the human power, the second one, you can't avoid it, is funding.

**Andreas Wendelboe** [85:55]: Yes.

**Lujo** [85:56]: For example, we are talking about very, very good idea. You see? So the fund, it will help you to buy maybe the equipment. Also, what I can advise Engineers Without Borders, they are engineers. So if you want to implement engineering project without talk about funding, guy, you will keep talking. You will keep talking. That is not engineering.

**Andreas Wendelboe** [86:37]: Yes. Yes.

**Lujo** [86:39]: Yeah. But I know and I understand that we are very, very limited resources, especially funding. And I understand it. But we will keep going. We will keep working very hard. But what I can say is that this vision will never die. Maybe I will die, but the vision will be still there. This is what I'm believing.

**Andreas Wendelboe** [87:11]: Yeah. That's amazing. And that also makes it a lot easier to fund. It shows that you're about action, that you want to get this stuff done, and that you're not just waiting for money, but that you actually go and you do it because your vision is so strong.

**Lujo** [87:35]: Guy, let me tell you something. After getting this equipment right now, I will assure you the monthly income for our organization, it will be about 14 million Tanzanian shillings.

**Andreas Wendelboe** [87:51]: Wow.

**Lujo** [87:52]: And we can do that. And we can do that because what we are doing is a business. Even me, I was wondering when Engineers Without Borders say that we care about sustainability of women, but what I care is how they can increase their income. What matters is that also how they can do sustainable projects. As you see, we have already transformed agricultural products. So it is like I'm wondering when you say about sustainability, we are already saying that we are transforming, and again, you are asking about sustainability. It is like I'm not understand that.

**Andreas Wendelboe** [88:49]: Yeah. I thought it was ridiculous too. Yeah. But it seems like the group is aligned now. And I don't think that I don't think it will be an issue going forward. So I think.

**Lujo** [89:11]: Yeah. And good issue is let me tell you one story. Last year, I have applied for American universities. You see?

**Andreas Wendelboe** [89:24]: Oh, okay. Yeah. Where?

**Lujo** [89:25]: Yeah. And one of the universities was San Diego. And the second one was St. Louis University in Missouri. Do you see? But unfortunately, our country was banned for traveling to United States.

**Andreas Wendelboe** [89:46]: Because of Trump?

**Lujo** [89:47]: According to the policy of Trump. Yeah. Trump is very global issue. It is like climate change right now. But yeah. My dream was like what I can say, my dream was like positive. But according to me, I normally believe in a process that maybe this issue has occurred for a special purpose. So it is my rather than giving up, I said I will work very hard and I will do better. Yeah. Yeah. And right now, this projects like what I'm doing, there is a lot of organizations in the United States have shown their interest. But the bad news about American organization, oh, guy, they need you to pass through many, many, many, many reviews. They need a lot of documentations. It is like banks. When you don't have money, they don't want you. But once you get the money, they keep coming to you. Can you come and borrow to our bank? I know how life goes. So what I believe is one day, people will come and chase me.

**Andreas Wendelboe** [91:33]: Yes. Yeah. And I think.

**Lujo** [91:36]: They will come and they will.

**Andreas Wendelboe** [91:37]: I think you will learn so much more by being the boss of a Rise and Shine than you will by going to university in America. I think just being out in the real world, doing real things, you learn so much.

**Lujo** [91:58]: For sure. Yeah. And that is what I believe in that. And I will keep learning. I will keep learning. And another issue is the times keep going. Just keep older enough. For example, now I'm about 30, 34. How old are you?

**Andreas Wendelboe** [92:22]: I'm 27.

**Lujo** [92:25]: Oh, 27. Yeah. You are much younger than me. So I am 24. Do you see?

**Andreas Wendelboe** [92:36]: Yeah.

**Lujo** [92:38]: And this world needs you must have a good legacy in this world when you live one day because the time is very limited. So it is better to work very hard to accomplish your dream. No one will come and give you a messy. Just it is up to you to work very hard, accomplish your dreams. Like what you are doing, for example, you have developed a lot of software, and you are using these AI tools in a very good way. So this is what I appreciate about you. I think you will do a lot of things and very big things. But what's important is solving problem to the community, yeah, to the people surrounding you when you help them, like what we are doing right now. For example, we have used about two hours talking about people. Do you see? Yeah. It pay. It pay.

**Andreas Wendelboe** [93:48]: Yeah. Yeah. I love that. I love helping community and yeah, solving problems.

**Lujo** [94:01]: For sure. And one day, the generation will write in their history that this guy may be Lucian and Andreas. They helped us in Tanzania. They loved this one. Do you see? That is what matters than millions of money. Do you see? When one day someone will say that Andreas helped me to solve this one, maybe somebody helped me to do this one. That is much worth than millions of money in your account.

**Andreas Wendelboe** [94:38]: I agree. Totally agree.

**Lujo** [94:41]: Yeah. Yeah. Yeah. So Bwana Andreas, thank you very much. I see you are very, very you are very, very good man.

**Andreas Wendelboe** [95:03]: You too, Lujo. You too.

**Lujo** [95:05]: Yeah. I really appreciate you, guy.

**Andreas Wendelboe** [95:10]: Yeah. This is going to be great. This is going to be a good partnership. I look forward to get started building some stuff.

**Lujo** [95:27]: Yeah, sure. We will do that. Yeah. Are you still a university student?

**Andreas Wendelboe** [95:36]: Yeah. But I'm on my last semester. So I finish in January.

**Lujo** [95:44]: Wow.

**Andreas Wendelboe** [95:45]: Yeah. So.

**Lujo** [95:47]: Masters or?

**Andreas Wendelboe** [95:48]: Yeah. Master's in AI and robotics.

**Lujo** [95:55]: Wow. So you are in the right field.

**Andreas Wendelboe** [96:00]: Yep. Yep. I use AI all the time. Yeah. And.

**Lujo** [96:08]: And our world right now is in AI right now. Yeah.

**Andreas Wendelboe** [96:13]: Yeah. Yeah. I work for an AI company as well.

**Lujo** [96:18]: Wow. Wonderful.

**Andreas Wendelboe** [96:21]: Yeah. So I stay up to date with the AI industry and what's happening and stuff. And it's pretty cool.

**Lujo** [96:36]: That is very cool. Yeah. You are in the right field right now because everything right now is need machine learning, need AI. Yeah. You have chosen a very, very good field. Even me, I applied for master's in data analytics engineering, something like that. But at that time, I don't care about those titles. I was just looking for how I can go and study and help my community in a very good way. And maybe if good wish, maybe one day, I will turn my mind to start looking for European universities because also you provide very, very good education. Because America right now is like, "Oh, I'm going to hate America right now."

**Andreas Wendelboe** [97:54]: Yeah. So we do it. We also hate America and Europe right now. We're not good friends with them. But yeah, you should consider studying in Denmark as well. The schools are very good, and everyone speaks English.

**Lujo** [98:13]: Whoa.

**Andreas Wendelboe** [98:13]: Yes.

**Lujo** [98:16]: Are they providing scholarships? Maybe university? Are they have a special scholarship maybe for Africans?

**Andreas Wendelboe** [98:25]: I'm not sure. I'm not sure, but I could research it for you. I'm friends with a lot of Chinese people who came to Denmark to study. But yeah, I don't know.

**Lujo** [98:49]: But I heard about Danish scholarship. They said that it is Danish ambassadors in Africa. They offer scholarship for East Africa, something like that. But it was past years. I didn't focus about that information. But I will do research, but not now. Maybe when I set up my organization, when we will start operating very good way, I will start looking for universities, for master's. Yeah.

**Andreas Wendelboe** [99:33]: And I think the fact that you have made a rise and shine and that you're doing all this good stuff for the community, I think will also help getting a scholarship. It shows that you care and that you're a good person and stuff. So maybe that will help. Yeah.

**Lujo** [99:56]: Yep. Also, it will be a very interesting story. Maybe I was keep learning from someone who is doing master's in machine learning and AI. So it will be very interesting story.

**Andreas Wendelboe** [100:17]: Oh, yeah. Definitely.

**Lujo** [100:19]: Yeah. So you must be my teacher right now because and what I'm learning will keep in actions and solving people problems. Yeah.

**Andreas Wendelboe** [100:32]: Yeah. We will learn a lot from each other.

**Lujo** [100:38]: Yeah, for sure. Yeah.

**Andreas Wendelboe** [100:40]: Yeah. I can teach you about AI, and you can teach me about Africa and Tanzania and what it's like building a business there.

**Lujo** [100:53]: Yeah, for sure. And I hope one day you will visit Tanzania, right?

**Andreas Wendelboe** [100:59]: Oh, yes. I'm definitely coming to visit. Maybe next year, maybe in two years. I don't know if you know it, but we have some good friends of the family, and his family, they have a nonprofit in Tanzania that does hygiene stuff. It's called Majisafi Group. I'll show you.

**Lujo** [101:42]: Majisafi Group.

**Andreas Wendelboe** [101:43]: Yeah. They're near Lake Victoria. I just sent it in the chat.

**Lujo** [102:01]: Wow.

**Andreas Wendelboe** [102:08]: I don't know what their address is in Tanzania.

**Lujo** [102:12]: They're found in Lake Zon, right? Lake Zon.

**Andreas Wendelboe** [102:17]: Okay.

**Lujo** [102:21]: Oh. Majisafi improve livelihood in Tanzania.

**Andreas Wendelboe** [102:41]: Mara region.

**Lujo** [102:44]: Mara region. Oh, okay. Mara region.

**Andreas Wendelboe** [102:53]: Shiraati.

**Lujo** [102:54]: Yeah. Shiraati. Yeah. Shiraati. Yeah. This is really Mara. It is Mara.

**Andreas Wendelboe** [103:07]: Yeah. So yeah, our family is good friends of Majisafi Group. So I might come visit Majisafi and then also visit you.

**Lujo** [103:24]: Yeah, sure.

**Andreas Wendelboe** [103:25]: That would be cool.

**Lujo** [103:29]: Yeah. And Mara region is where the big national park and the world-renowned park, it is called Serengeti. Have you heard about Serengeti?

**Andreas Wendelboe** [103:44]: Oh, yes. Very famous. Very famous.

**Lujo** [103:48]: Yeah. But also where we found Manyara is near Arusha. It is a gateway to the Ngorongoro Crater and Serengeti. So we are very near with Mara, but we are different people. We are different people. Yeah. Most of them who founded Mara are Maasai. And there is this tribe called Wakuria. But in Manyara, Iraq, Maasai, Barbaik, Hazabe. I have sent you in our WhatsApp group about Hazabe, the Bushmen, who are founded who are found in Manyara, Manyara region. So it will be good for you when you are doing a remote community project and also you are visiting the right field. It will be a very, very interesting journey to you. When you visit, also you will keep learning a lot about communities.

**Andreas Wendelboe** [105:10]: Yeah.

**Lujo** [105:11]: Yeah.

**Andreas Wendelboe** [105:13]: I can.

**Lujo** [105:13]: You know, traveling, through traveling, you will keep learning many things.

**Andreas Wendelboe** [105:18]: Yeah. Yeah. I can't wait.

**Lujo** [105:20]: Yeah. Yeah. You are warmly welcome.

**Andreas Wendelboe** [105:25]: Thank you. Thank you very much. But that's great.

**Lujo** [105:37]: Yep.

**Andreas Wendelboe** [105:41]: I will take a look at all this stuff that we talked about, and then I will message you with the details.

**Lujo** [105:56]: Yeah. Thank you very much. I will appreciate it. And I'm looking forward. Also, I will do my best. This week, we have started installing electricity in our production site. So last week, we have finished making those floors. And I hope also this week, I will send you an update in our WhatsApp group so you will see what we have done.

**Andreas Wendelboe** [106:28]: That's great.

**Lujo** [106:28]: Also, during nighttime, I will work hard to pass through all this stuff that we are talking right now. And I see you will help me with a lot because for this time, it is like I'm very, very tighted. But because you are here, you will help me a lot. Also, we will make this transformation from these frameworks to HTML and CSS. Yeah.

**Andreas Wendelboe** [107:04]: Yeah. Remember to also sleep at some point.

**Lujo** [107:10]: Yeah, for sure. After finish here, I will just go to take my bath. Then it will be direct to sleep.

**Andreas Wendelboe** [107:19]: Good. Yeah. Okay.

**Lujo** [107:28]: Thank you very much, Bwana Andreas.

**Andreas Wendelboe** [107:31]: Thank you very much, Lujo. I'm very excited.

**Lujo** [107:38]: Me too. Me too. I'm very excited, guy, to meet somebody like you. Very good man, very supportive.

**Andreas Wendelboe** [107:46]: Thank you.

**Lujo** [107:48]: I really appreciate. Yeah.

**Andreas Wendelboe** [107:50]: Thank you. Okay.

**Lujo** [107:53]: Thank you very much.

**Andreas Wendelboe** [107:54]: Yeah. Perfect. Well, good night.

**Lujo** [107:58]: Yeah. You too. Take a good night.

**Andreas Wendelboe** [108:02]: Bye-bye.

**Lujo** [108:03]: We'll keep chatting, right?

**Andreas Wendelboe** [108:04]: Yes.

**Lujo** [108:05]: Yeah.

**Andreas Wendelboe** [108:06]: Definitely.

**Lujo** [108:06]: Thank you very much.

**Andreas Wendelboe** [108:07]: Bye.

**Lujo** [108:07]: Bye-bye.

**Andreas Wendelboe** [108:08]: Bye-bye.
