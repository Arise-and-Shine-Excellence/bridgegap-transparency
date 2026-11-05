# Engineers Without Borders Project Scope

- **Date:** 2026-10-13
- **Participants:** Andreas Wendelboe, Lujo
- **Source:** Wispr Flow meeting `cb0555c8-ee76-49b9-b39f-516da80abbeb`

## Summary

EWB Denmark team welcomed two new members, aligned with Lujo on the MAHS project plan ahead of the 15 November project scope workshop, then discussed moving Bridge Gap Transparence to AI coding agents and HTML/CSS.

### Team Update and Project Plan
- Two new members joined, making a full group of five: one focused on finance and budgets, one a retired chemical engineer
- Project is moving from design into defining the project scope, a document of roughly three pages
- Full-day scope workshop set for Sunday, 15 November; Lujo joins for one hour at the end to review results
    - Agreed slot is 2 to 3 o'clock Danish time; daylight saving shift may change the Tanzania offset to two hours
- Lujo can attend if given updates about a week before

### Short-Term Progress in Tanzania
- Lujo used family and friends' funding (about 2.5 million Tanzanian shillings, roughly 6,000 kroner) for equipment
    - Electric motor for mixing, laptop, soap cutter table, about five extra molds, drying racks
- Cutter table lets one person cut up to 1,000 soap bars a day; target is about 3,000 bars per month
- Lujo designs equipment on paper and has local welding garages fabricate it
- Mixer advice: run slowly to avoid mixing air into the soap; motor is rated for cement and concrete

### Definitions, Power, and Scope Ideas
- Lujo treated short-term as about three years; EWB means up to one year, longer counts as long-term
- Power cuts are mostly brief, roughly monthly, and planned outages are announced; heavy rain causes more
    - Battery, solar, or generator backup only needed if outages are frequent
- Main need is separate permanent rooms for production, drying, and storage instead of one wooden shed
- Lujo suggested scoping at Manyara region level (about 1 million people) rather than Mbulu district (about 300,000)
    - Households spend about 45 to 48 thousand Tanzanian shillings per year on soap and hygiene products

### Lujo Background and Needs
- Lujo founded Arise and Shine Excellence and Mama African Hygiene Solution, making soap from sunflower sludge
- Production site is in Yaeda Kati village (spelling uncertain), Mbulu District, Manyara region
- Biggest gap is expert technical guidance on formulation that meets national and international standards

### Software: Bridge Gap Transparence
- App is built in Python with Streamlit (heard as 'Streamlight'), Supabase, GitHub, by copying code from Claude and Gemini
- Andreas Wendelboe demoed coding agents (Claude Code, Codex CLI, Gemini) that read and edit files directly; about $20 a month plan suggested
- Andreas Wendelboe recommended dropping Streamlit for plain HTML/CSS and a responsive website over a native app
    - App stores add review overhead; build an app only if phone features demand it
- Requested feature: scanning products with a phone camera or QR code in the Biashara Yako Mkononi Mwako module

### Next Steps
- (Speaker 1) Send Lujo the invite for the 15 November scope workshop at the right local time
- (Lujo) Share installation update and demo of the new equipment in the WhatsApp group
- (Lujo) Send a list of purchased items and a brainstorm of things to fund in the project scope
- (Lujo) Send Andreas Wendelboe the app goals, demo link, login, and GitHub access via WhatsApp before Monday

### Decisions Made
- EWB will pursue both a short-term track (within a year) and a long-term funded track
- Project scope will include a new permanent production and training center

## Transcript

**Speaker 1** [00:01]: Så er der svært at være i bænkstykke.

**Andreas Wendelboe** [00:05]: Ja, men det er rigtigt. Så får jeg noget motion til gengæld.

**Speaker 1** [00:09]: Ja.

**Andreas Wendelboe** [00:10]: Andreas?

**Lujo** [00:11]: Hello.

**Andreas Wendelboe** [00:20]: Kan I ikke høre mig?

**Speaker 1** [00:22]: Ja, vi kan godt høre jer.

**Andreas Wendelboe** [00:23]: Super.

**Speaker 4** [00:25]: Kan I også høre mig?

**Speaker 1** [00:27]: Ja.

**Andreas Wendelboe** [00:27]: Yes.

**Speaker 4** [00:28]: Hej.

**Speaker 1** [00:29]: Hej.

**Andreas Wendelboe** [00:31]: Hejsa.

**Lujo** [00:33]: Jeg tror, den her gang skal vi være gode til at afbryde ham, hvis han løber ud ad en tangent.

**Andreas Wendelboe** [00:38]: Ja.

**Lujo** [00:40]: Så det ikke bliver et 2-timers møde.

**Andreas Wendelboe** [00:43]: Jeg tænker at tage fat i de spørgsmål, han især har. Altså, øh... Could we therefore please clarify the following? Det han skrev i sin mail. Jeg ved godt, jeg har kort svaret ham, men altså, at vi lige drøfter de 4 ting i hvert fald.

**Lujo** [01:02]: Ja.

**Andreas Wendelboe** [01:02]: Ja. I hvert fald helt konkret, ikke?

**Lujo** [01:09]: Jeg tror kun, vi mangler Kristoffer, selvfølgelig, Lujo.

**Andreas Wendelboe** [01:31]: Hello.

**Speaker 1** [01:34]: Hallo.

**Andreas Wendelboe** [01:34]: Hello, Lujo. Ja, hello.

**Lujo** [01:39]: How are you?

**Andreas Wendelboe** [01:42]: I'm fine, I'm fine. How about you guys?

**Lujo** [01:46]: Very good.

**Andreas Wendelboe** [01:47]: Good, thanks. Great to see you again.

**Speaker 1** [01:53]: You too.

**Andreas Wendelboe** [01:55]: Yep. We're waiting a little. We have, yeah, some news among other. We have a fifth member now joining the team, Kristoffer. Another Kristoffer. He will mainly take part of the financial part, also for the, for example, for the budget when we go and search for a long-term fund for the project. Yeah.

**Lujo** [02:38]: Has he said that he's coming to this meeting?

**Andreas Wendelboe** [02:41]: Yes. I can ask him if he's on his way.

**Lujo** [02:54]: Maybe he's having a hard time logging in with his new account.

**Andreas Wendelboe** [02:58]: Yeah. What time is now, to your side?

**Lujo** [03:09]: Like 5 p.m.

**Andreas Wendelboe** [03:12]: 5 p.m.?

**Lujo** [03:14]: Yeah. So is that one hour behind you?

**Andreas Wendelboe** [03:20]: Yeah, it seems it's one hour behind me. Yeah.

**Lujo** [03:29]: How is the weather in Tanzania, like, right about now?

**Andreas Wendelboe** [03:34]: Oh, we were expecting El Niño, but it is too hot up to now, up to date. It is too hot. No rain.

**Lujo** [03:46]: Yeah.

**Andreas Wendelboe** [03:47]: Yeah.

**Lujo** [03:49]: So you guys also get El Niño? I only thought that was, like, South America.

**Andreas Wendelboe** [03:57]: Yeah, yeah. We were expecting to get El Niño. We were expecting at the end of September and at the beginning of October. But fortunately, we thank God, nothing, nothing occurred. But we are waiting, we are waiting for it.

**Lujo** [04:14]: Yeah. Yep.

**Speaker 1** [04:17]: What would happen if you get El Niño? Get a lot of rain, or?

**Andreas Wendelboe** [04:22]: A lot of rain. I see it is like no going out, just you will stay in your homes because a lot of infrastructure will be destroyed. So we are afraid, we are afraid for El Niño.

**Speaker 1** [04:39]: Okay.

**Andreas Wendelboe** [04:40]: Yeah.

**Speaker 1** [04:42]: Do you also get some hurricanes then, or when it happens?

**Andreas Wendelboe** [04:47]: No, no. In Africa, no. Maybe to those people who live near to the oceans, like Dar es Salaam, maybe. Or Tanga. Yeah. But in Africa, I don't know. We didn't experience any type of, like, hurricanes. It's called a hurricane.

**Speaker 1** [05:12]: Hurricane, yeah.

**Andreas Wendelboe** [05:13]: Yeah. We didn't experience it.

**Speaker 1** [05:16]: Okay.

**Andreas Wendelboe** [05:17]: Yeah. Yeah, I think we are all here. Just mentioned to you, Lujo, that Kristoffer is our new group member. So maybe you would like to make a little presentation of yourself? I think we cannot hear you.

**Speaker 1** [05:47]: You're muted.

**Andreas Wendelboe** [05:51]: There we go. Perfect. Yeah, so my name is Kristoffer. I'm 26 years old. I recently graduated with a major in business. I also have a bachelor within engineering. And yes, I recently joined Engineers Without Borders. And I talked with Anna about this project, which I think sounds really interesting. And yeah, I'm very motivated to see where this goes and how I can help with this.

**Speaker 1** [06:32]: Yeah. Wonderful. Thank you.

**Andreas Wendelboe** [06:38]: You, and as I remember, you met each other last time, Martin and Lujo, right?

**Speaker 1** [06:45]: No, I don't think so.

**Andreas Wendelboe** [06:47]: Okay. Then I remember. But yeah, we have two new group members. Yeah. Would you also like to present yourself, Martin?

**Speaker 1** [06:57]: Yeah, my name is Martin. I'm 65 years old. I'm retired this year from my engineering career. And this is the second project in Engineers Without Borders which I am participating in. But still, it also sounds very interesting for me to help you, if possible.

**Andreas Wendelboe** [07:25]: Wonderful. Thank you. Thank you very much. And just to let you know, Lujo, usually, as I understand, it is typical that you are five members in a group for a project in Engineers Without Borders Denmark, until Martin and Kristoffer join, we'll be three. But now we are a full group. So that is super good. Yeah. We had an email correspondence, and you have sent some questions to us, Lujo. Specifically question one, two, three, and four. We have briefly answered you, but I think this is like the starting point and main points on the agenda today. Yeah. So the first question here is, maybe you'd like to present yourself, but I can see you asked, what is Engineers Without Borders Denmark's current plan for the MAHS project? The current plan is that we are about to go into, from a design phase, into defining the project scope, as also mentioned in the email. And we have agreed in the group to meet on Sunday, the 15th of November, full day to define this. What I can understand on Anna, our project advisor, is that this project scope is typically around three pages only, so it's really comprised. But the intention of this is to have a clear definition of what is the project scope. And this is both for you, for us, and also for later when we will search for funds for the project. Yeah.

**Lujo** [09:54]: Yeah, thank you.

**Andreas Wendelboe** [09:58]: Is that clear? Do we have any? Yeah, anything to add?

**Lujo** [10:05]: Yeah. Yeah, it is clear. I see the planning. It is open, so this is what I was looking for, yeah.

**Andreas Wendelboe** [10:17]: Yeah.

**Lujo** [10:17]: To get the clear pathway. Yeah, like where we are now, where do we expect to go. Something like timelines, do you see?

**Andreas Wendelboe** [10:28]: Yeah.

**Lujo** [10:28]: Because without timelines, without deadlines, it's like it is not encouraging. So thank you very much for these new members, Kristoffer and Martin, to join in this project. It is really, really encouraging. And I hope we are going to do very, very amazing work here in Tanzania, East Africa. So thank you, and warmly welcome, guys. Yeah.

**Andreas Wendelboe** [10:59]: Thank you.

**Speaker 1** [11:00]: Thank you.

**Andreas Wendelboe** [11:01]: Can I ask you here, we have already thought about that during the end of this day where we will have this, we call it a workshop, where we will sit down together and define this project scope. We also invite you to join in for, we have said, one hour aside for this. Are you available on this day, or especially from 3 to 4 o'clock p.m. Tanzania time?

**Lujo** [11:33]: Sorry, are you talking about today, or?

**Andreas Wendelboe** [11:37]: Yeah, so when we meet on the 15th of November to define this project scope, we have talked about it would be good to invite you for a meeting because we will work the whole day and then ask you, let's say, in general, present to you what we have got as results as, yeah, for this project scope.

**Lujo** [12:02]: Yeah, I will be available if I will get updates maybe one week before.

**Andreas Wendelboe** [12:08]: Yeah.

**Lujo** [12:09]: Yeah, I will be able to arrange my timetable and to schedule the plan. So I will be available, yeah.

**Andreas Wendelboe** [12:17]: Super. That's good to hear.

**Speaker 1** [12:20]: Kristoffer, you mentioned a time on the 15th. In Denmark, we have something which we call daylight saving time. So we go back, in the end of October, we go back again to, yeah, to normal time. I don't know, do we have anything like that in Tanzania also? Or do we have the same? You don't change time during the year, go back or forward one hour? Oh, you look very suspicious. I don't think you do it. But okay, anyway, we will forward you an invitation for the right time, and I think it will be transferred to your correct time also. But it's, I don't know exactly, are we going back or are we going forward in two weeks from now?

**Andreas Wendelboe** [13:13]: Back.

**Speaker 1** [13:14]: Back. Okay. So the difference will be maybe two hours then.

**Andreas Wendelboe** [13:20]: Yeah, that's a good point, Martin. We will send you an invite for this time. We have agreed to what is from 2 o'clock to 3 o'clock Danish time. Yeah.

**Speaker 1** [13:37]: Yeah.

**Andreas Wendelboe** [13:41]: Yeah. I will go to your point two of your questions. Do we still intend to proceed with the short-term implementation phase that we discussed? Yes, we do. Yeah, so that's the short answer. We go on this long-term track and we go on this short-term track. You mentioned that you have been in Dar es Salaam to purchase some equipment. Is that right?

**Lujo** [14:15]: Yeah, it is right. Yeah. I traveled to Dar es Salaam to buy some equipment to optimize the production process. As we discussed earlier in our previous meeting, we were talking about to optimize the production process, like reducing the time consuming, like shifting from the local way of mixing. We were proposed to have electric motor that will help those women and the youth in a production site, and to speed up the process of mixing. Also, in our previous meeting, we were talking about to speed up the process of drying our soap bar of our soap bars after cutting. We were talking about drying racks, I think you remember. Also, we were talking about to optimize those process of cutting soap, because before we were using the thread, it was time consuming. It will need two people, one person to stand at the behind of the block of our soap bar, and another one to pull it. But right now, we have designed something like a table that will enable only one person to cut even 1,000 soap bars in a day. So we have designed that cutting table, and they are still at the workshop there, at the garage. So when it will finish, I will show you and I will demonstrate to you how it is working. So the good thing is we have secured some fund from our friends, our family members, yeah, to purchase those equipment. So it was the purpose of my traveling to Dar es Salaam. Yeah.

**Andreas Wendelboe** [16:36]: Wow. That's really cool.

**Lujo** [16:40]: Yeah.

**Andreas Wendelboe** [16:42]: Congratulations.

**Lujo** [16:44]: Yeah.

**Andreas Wendelboe** [16:45]: Was it expensive?

**Lujo** [16:50]: It's about 2.5 million Tanzanian shillings.

**Andreas Wendelboe** [17:00]: I got to convert that. It's like 6,000 kroner. Not bad. And is the mixer, is that like working right now?

**Lujo** [17:17]: I'm not really fabricated to that, because we are designed like to frame a framework to make a frame.

**Andreas Wendelboe** [17:26]: Yeah.

**Lujo** [17:28]: Yeah, so it will hold. Then we will put that container at the bottom, so it will mix. But right now, we just have a motor. Yeah, but we are waiting for those tables when ready. We will fabricate in one table in a frame, so it will be easier to operate. It is something like that.

**Andreas Wendelboe** [17:57]: Nice.

**Lujo** [17:58]: Yeah.

**Andreas Wendelboe** [17:59]: That's really cool.

**Lujo** [18:00]: And I hope this week I will finish every work of installation of this equipment, and I will share the update, I hope, in our WhatsApp group, yeah.

**Andreas Wendelboe** [18:12]: Yeah, that would be great. And it's nice to see that you're like moving ahead fast and stuff. I think we, for the meeting that we have in November, we need to know like what do you still need from us? Like what do you want funding for and stuff like that? Because I guess now we can't give funding for the motor, but maybe we can get funding for something else. So that would be interesting to hear is like what can we help you with going forward now that you have already taken some of the first steps?

**Lujo** [18:56]: Yeah, I think important issue in this project is to get the, it is like expert recommendations. For example, Kristoffer here is a chemical engineering, so we need to optimize that production in very official ways rather than locally, because I am not.

**Andreas Wendelboe** [19:22]: Can I say it? Of course. Martin here is also a chemical engineer, and he has many more years of experience than me, so. And we are all bright minds, so yeah. But we will all contribute.

**Speaker 1** [19:41]: Yeah, I was just going to ask a question about the mixer. I think you pointed that it's going to be like in this direction, not directly down. And also the speed of the mixer, I think it's quite important that it should not be too fast, because otherwise you have the risk that you are mixing air into the fluid, into the soap. So it should be almost like what you do, what a person is doing, but then just in a steady way. Just some technical things.

**Lujo** [20:17]: Yeah, sure. Yeah, something like that, yeah. So the mixer that we purchased is the specification that they mentioned in that product is they said that it is special for mixing cement and concrete, something like that. So we bought something like that. So I think it is also good in our experience in Tanzania. They said that it is also good for mixing soap.

**Speaker 1** [20:54]: Okay. Sounds good.

**Lujo** [20:57]: Yeah.

**Andreas Wendelboe** [21:00]: Great. I don't know if it's possible, if you could like make a list of stuff you need help with or things you would like to buy, so we get like an overview. I assume you have a bunch of good ideas, so it would be cool to see that. You don't have to make it like super official, like you did with the project idea template and stuff. It would just be cool to have like a list of things, both physical and software, if that's something you need.

**Lujo** [21:41]: Yeah, you know, maybe I don't know if it is my understanding or, you know, when I'm talking about short-term implementation of project, to my understanding, I was talking about maybe after maybe three years from now, after three years, what should I need to be accomplished, do you see? But for long-term, when I'm talking about long-term, I think we will not going to use this called process methods, because long-term, when we are talking about long-term, is like make a soap manufacturing industry. For example, if our industry, if it will be able to accommodate the demand of all Manyara regions, something like that, or all Tanzanians' small farmer holder, do you see? So you can implement the long-term process using this called process method, because this called process method, even this short-term equipment that we have purchased, they are still in a bottleneck. It's like the speed is very low, because you can, because nowadays there is those machines like at the same time it produces the soap bars, it branding at the same time, it is an automatic machine, do you see? So, but this short-term is just I'm looking for like things, these small things like motors, local branding, like we are putting temp, do you see? We are still implementing in a local way. But a long-term process, I'm looking for something like, for example, if we need to produce 50,000 soap bars at month, do you see? We can implement it using this local equipment.

**Andreas Wendelboe** [23:50]: Yeah.

**Lujo** [23:52]: Yeah.

**Andreas Wendelboe** [23:54]: But I think it's good to hear your definition of short-term, because short-term for us, or at least for me, that is here and now or at the end of this year. So that's why I said no in your email. Long-term is, I would say it is typically within two years, something like that, or maximum three years.

**Lujo** [24:24]: Yeah.

**Andreas Wendelboe** [24:25]: Yeah, but that's also because we're thinking from an organization, like from Engineers Without Borders side, like project, but you're thinking business. Like your business short-term is, of course, three years. That makes sense. Your long-term is longer than that. But since we're like on a project basis, for us, short-term is like up to a year, and anything over that is long-term. So I think just our definitions are a little bit different. It's totally fine. But now you know, like when we say short-term, that's what we mean, and long-term, stuff like that. And we're, of course, we would love to help you plan this like long-term project, like how you'll do it in the future, in like 10 years or whatever. That's, of course, something we can help you with and maybe find funding for and all that sort of stuff. So don't worry about short-term, long-term. It's more like we want to both help you find funding and also like help you right here, right now with stuff. That was basically what we mean by long-term, short-term stuff.

**Lujo** [25:42]: Oh, I really appreciated the definition, because I was thinking about the short-term, I was thinking about maybe two years, three years, something like that. Yeah. Thank you. Thank you for definition, guys.

**Speaker 1** [26:00]: Good that we have clarified that.

**Andreas Wendelboe** [26:08]: Yeah, and in the last questions, question four in your email, I think we have clarified, we have got the clarification from you regarding any doubts and that if you have, if all the group members, if you have something to ask here, please do so. But I think it's quite clear.

**Lujo** [26:40]: Yeah.

**Andreas Wendelboe** [26:46]: Can you please repeat for me, Lujo, you said that what you have got from when you went to Dar es Salaam, it is the electrical motor, right?

**Lujo** [26:58]: I put electrical motor.

**Andreas Wendelboe** [27:00]: And then what else?

**Lujo** [27:03]: Also, I bought my computer, so I think in that part of software with Andreas, I think I've bought a good laptop. So my electricity is off right now. That's why you see, yeah. Sorry.

**Andreas Wendelboe** [27:20]: Yeah.

**Lujo** [27:21]: Yeah, I've cut the electricity. So I have bought a good laptop computer.

**Andreas Wendelboe** [27:28]: Congrats.

**Lujo** [27:29]: I see the electricity is back again. Thank you, guys. So I bought a laptop computer so that I can keep programming that bridge gap transparent and so I'm really, really excited to work very, very hard, guys, to accomplish the goals of this project. Yeah. Also, here, just here in Bolo, we have bought those raw materials like to design that soap cutter table. So they are fabricated at the garage right there. So from that fund, so is how we are using that fund. So we are designing tables. We call it a soap cutter table. Also, we are designing an extra soap mold, about five of them. So because we want, yeah, we want in every month to reach about 3,000 soap bars. Yeah.

**Andreas Wendelboe** [28:44]: Who helps you, like the cutting board or the table board, soap board, and the molds and stuff? Who helps you make that? Is that yourself, or do you have someone else, like a handyman?

**Lujo** [29:00]: I'm just designing. I'm designing and drawing the design, then bringing to welders and told them, can you design something like this in these dimensions? Yeah. But I'm designing and bring them to the welders.

**Andreas Wendelboe** [29:19]: Yeah. Are you designing it in CAD or like on paper, or how do you do it?

**Lujo** [29:27]: Just, for example, in a paper.

**Andreas Wendelboe** [29:33]: Yeah.

**Lujo** [29:35]: Yeah, in a paper. For example, this was I was designing the table.

**Andreas Wendelboe** [29:41]: Okay, yeah. If you could send pictures of that, we would be very interested in seeing how it works and stuff.

**Lujo** [29:52]: Yeah. Yeah. Hope this week everything it will be finished, then I will send you the demonstration. Yeah.

**Andreas Wendelboe** [30:03]: Nice. And you mentioned you had also bought like drying racks.

**Lujo** [30:08]: Yeah, even drying racks. Yeah. We have designed it. Also, they are there to the welders.

**Andreas Wendelboe** [30:18]: Okay.

**Speaker 1** [30:20]: Do you have employed welders, or how does that function?

**Lujo** [30:25]: No, there are special garages here in our towns.

**Speaker 1** [30:29]: Okay.

**Lujo** [30:30]: Yeah. It is like a workshop, somebody workshop. So I'm bringing it right there. And I'm paying there.

**Speaker 1** [30:38]: Yeah. That's good to hear.

**Andreas Wendelboe** [30:41]: Yeah.

**Speaker 1** [30:42]: Lujo, you just had a power failure. Does it happen very often in Tanzania? You just had an electricity failure. You lost power.

**Lujo** [30:59]: Yeah, normally frequently we are losing power, but sometimes after two hours when they cut off, right after two hours, it will be back again.

**Andreas Wendelboe** [31:11]: All right.

**Speaker 1** [31:12]: I don't know if how vulnerable is the mixing of soap if suddenly now you're using, going to use an electrical motor. If it stops because there is no power, what happens to the soap mixture? Can you still use it? Do you need any kind of backup?

**Lujo** [31:38]: Yeah, absolutely. We need something like a backup because, but normally in Tanzania, nowadays they have improved this service of electricity. For example, it can be occurred after only one hour after they cut, then the electricity to be back. And normally during night, the electricity is on all the time. Yeah. But also it needs timing because they have improved the services. Maybe they will inform you, maybe tomorrow we will cut electricity after certain hours. So it is up to you to do timing. This is what we are doing. Yeah.

**Andreas Wendelboe** [32:31]: Do you know like how often? Like is it every single day, or is it like once a week, once a month?

**Lujo** [32:41]: Normally it will occur when they need to do their improvement in their infrastructure normally.

**Andreas Wendelboe** [32:47]: Yeah, but like what does that mean? Like how often would you say? Like it just happened now? Did it happen yesterday?

**Lujo** [32:56]: For example, this week, I can say monthly, monthly, because it depends, it depends.

**Andreas Wendelboe** [33:05]: Okay. So only like once a month. Yeah.

**Lujo** [33:11]: Yeah, something like that. But it's normally by emergency. Maybe during heavy rain also, we are expecting to have a lot of cut off of electricity. So it depends, it depends. Yeah.

**Speaker 1** [33:25]: Why does the heavy rain make that breakdown in electricity? I'm just curious. I mean, I don't see why.

**Lujo** [33:37]: You know, in Tanzania, we are just using the hydroelectric power only. So we are depending on that. Also the issue of infrastructure during heavy rain, a lot of infrastructure are distracted. Yeah. Also we are using those wooden pillars for electricity transmission. So during heavy rains, they are going to be distracted a lot.

**Speaker 1** [34:07]: Okay.

**Andreas Wendelboe** [34:08]: Yeah. And I'm still a little bit curious. Like your electricity just shut off for like 30 seconds, right? That wasn't like a two-hour delay, like you say.

**Lujo** [34:27]: Yeah, for example, today.

**Andreas Wendelboe** [34:29]: Yeah.

**Lujo** [34:29]: Sorry. For example, for today issue, today case is different. You will see it just cutting, then it's coming. So it's something like that.

**Andreas Wendelboe** [34:39]: Okay. And how often does that happen?

**Lujo** [34:43]: For example, for this week, we have experienced this day now.

**Andreas Wendelboe** [34:47]: Okay. So first time this week.

**Lujo** [34:51]: Yeah, something like that. But it is an unexpected issue, so.

**Andreas Wendelboe** [34:56]: Okay. Yeah, because.

**Lujo** [34:59]: It is not planned, so.

**Andreas Wendelboe** [35:02]: Yeah. Yeah, because if it happens enough, then we might need to see if we should get a battery or solar power or whatever to have the mixer or whatever running, even during blackout. But if it doesn't happen that often, then it's not necessary.

**Lujo** [35:26]: To my experience, I can say that electricity is not an issue. Because as you see right now, when this occurs, it is normally just because it is an unexpected issue. But if they are going to cut maybe for two hours for certain issues, for example, repair of their infrastructure, they will inform you the day before. You see?

**Andreas Wendelboe** [35:52]: Yeah.

**Lujo** [35:52]: But this issue of just 30 or one minutes, it is unexpected. Yeah.

**Andreas Wendelboe** [35:59]: Okay. That's good.

**Lujo** [36:02]: Also, we do have in Tanzania those automatic generators. Yeah, for example, for those who use those cell phone towers, they have automatic generators. Those in hospitals, also they have automatic generators, something like that for backup.

**Andreas Wendelboe** [36:24]: Okay.

**Lujo** [36:24]: Yeah. So it depends how big is our project. Yeah, because those automatic generators, it costs a lot of money in Tanzania. Yeah, it depends how big is your project. Yeah. Yeah.

**Speaker 1** [36:43]: I think as Martin said, I mean, it could be a problem with electricity. And I'm a quite good fan, I'm a quite big fan of the cold drying process because then you are independent of electricity and stuff. But I see that you also would like less manpower to be used, for example, with a motor as you have purchased now, right? Until then, I have seen the main problem would be the space for you because I see that if you could increase the space for the soap bars to during the drying process, then you could also improve, or let's say, optimize the drying process.

**Lujo** [37:33]: Yeah, for sure. For sure.

**Speaker 1** [37:35]: So for that case, I saw, for example, a good thing would be to have another house for you or where you could keep that area dry in case it's raining. But if it's sunny, then it would be good to put this soap bars outside, maybe. Yeah.

**Lujo** [38:00]: Yeah, sorry. Pardon, my network was low. Sorry, can you repeat?

**Speaker 1** [38:06]: Yeah, I just think that for the project scope, I think it would be we have also talked about it earlier also, but I think it would be a good idea to have to suggest to have an additional indoor place where you could that could be attributed to the drying process, for example.

**Andreas Wendelboe** [38:29]: Yeah. A new shed or something. Yeah.

**Speaker 1** [38:33]: Yeah.

**Andreas Wendelboe** [38:34]: Do you think is that something you would want, or do you think it would help, more room?

**Lujo** [38:40]: For sure, because even the first time when I contacted you engineers without border, Denmark, the main problem that I have submitted to you, it was to get it is like the empowerment hub, do you see? Yeah, because we are doing right now everything in one place, in a wooden shed, do you see? Production at the same time. It's like a store at the same time at one place, then drying at one place. So if we can get the fund to have separate rooms, the permanent rooms, like to have a store, to have a production area, I think, guys, you have enough information about the quality of your product. It need to have separate storage area, production area, and ready-made products in a separate room, do you see? Something like that. But right now, what we are doing in one shed, everything are done in one workplace.

**Andreas Wendelboe** [40:01]: Yeah.

**Lujo** [40:02]: Yeah.

**Andreas Wendelboe** [40:04]: Okay. I think that should be part of the project scope then, that we do a new permanent production and training center, as you say in your original proposal. That would make sense.

**Lujo** [40:19]: Yeah, yeah. For sure. Yeah. Also, the big idea in this project, we don't want to move really from this quality process because quality process is very, very simple. It is a very simple way of producing soap rather than those big industry. So when we are saying the empowerment hub or the empowerment center, do you see? It's like in Mbululu District, for example, right now, in our Mbululu District, we have two production sites. One is found in Yaidakati. Yaidakati is the place where those small farmholders, do you see? They are found in a large number in Yaidakati. So women have trained how to use their local available resources, like transforming beeswax into skincare creams, do you see? Transforming those sunflower sludge into soap bars, do you see? So we are doing our project by looking at different aspects, like the social impact. We are not just looking for a big manufacturing, the big manufacturing industry.

**Speaker 1** [41:54]: Can you repeat or spell for me, actually? What is that place?

**Lujo** [42:02]: Yaidakati. Yaidakati.

**Speaker 1** [42:05]: Have you told about that before? I think it's I don't remember that name at all.

**Lujo** [42:13]: Yeah, in Mbululu, we do have this. We have separated this area in villages, do you see? In wards. Afterwards, we do have what do you call it? There is something then it come district. After district, we have regions, do you see? Yeah. For example, our project is found Mama African Hygiene Solution is found in Mbululu District. So it is found in Mbululu District, Manyara region. But the village where our production is, is found in Yaeda, Yaedakati Village, which is found in Bashe, Bashe Ward, Dongobeshi. Dongobeshi, then Mbululu. So we normally use Mbululu District, Manyara region. Yeah.

**Speaker 1** [43:17]: I clearly remember Mbululu and Manyara. It was just the more specific places there you mentioned.

**Lujo** [43:22]: Yeah.

**Speaker 1** [43:23]: If you can write that, maybe I got it. If you already have written it, that's my mistake, but please.

**Andreas Wendelboe** [43:31]: You can put it in the chat if you want.

**Lujo** [43:33]: I will put it in the chat. You will see it. Yeah.

**Andreas Wendelboe** [43:40]: And so that's like the city where the production facility is, like the specific city.

**Lujo** [43:47]: It is Yaidakati. Guys, have you heard about those bushmens? Do you know about bushmens?

**Andreas Wendelboe** [43:56]: No.

**Speaker 1** [43:57]: No?

**Andreas Wendelboe** [43:58]: What's that?

**Lujo** [44:04]: You don't know about bushmens?

**Andreas Wendelboe** [44:07]: No.

**Speaker 1** [44:09]: Please tell us.

**Lujo** [44:11]: Yeah, bushmen are those guys who live in bushes. Up to date.

**Speaker 1** [44:17]: That makes sense.

**Lujo** [44:19]: They are found in Yaeda, Yaeda Valley. I will send you the clip in our WhatsApp group. Yaedakati Village. Do you see the electricity is also off again?

**Andreas Wendelboe** [44:34]: Yeah.

**Lujo** [44:35]: Yeah. Yaedakati Village. Okay. I have sent the name.

**Speaker 1** [44:44]: Thank you. Since it is our first meeting with Martin and Kristoffer altogether, I think it is also a good idea, maybe before we end the meeting, if you could present in general, tell a little bit about what you're doing, Lujo. I don't know if what you all think about that.

**Andreas Wendelboe** [45:14]: Yeah. Yeah, if you could describe it in a short way.

**Speaker 1** [45:20]: Yeah, just.

**Lujo** [45:22]: Okay. Thank you, guys.

**Speaker 1** [45:24]: Something.

**Lujo** [45:26]: Yeah, for sure. Yeah. So I will introduce myself. My name is Lucian John Quintini. I'm calling myself as a social entrepreneur found in Tanzania, East Africa. I have graduated a bachelor's degree in informatics. I graduated it from Sokoine University of Agriculture, the only one agriculture university that's found in Morogoro region here in Tanzania. So while there in university, of course, I have experienced many things about how can you integrate technology with agriculture. This is what I have learned from university. So after graduating my bachelor's degree in 2017, I committed myself to return back to the community where I have been raised. So I returned back in Mbululu District, where I am right now. So while in my community, I have identified different challenges in my community. I have seen how my mom was working very hard. And another issue about my mom was local brewers, do you see? She was good in making those local beers. So I have experienced how I use technology of brewing local foods into a beer. So from there, my mom was one of the great inspiration to me. So I said, now it is my time to use my educations, my skills, to solve these challenges into opportunities. So is where I founded an organization called the organization called Arise and Shine Excellence. Arise and Shine Excellence. It contain names. Arise is like you are telling someone, "Arise." Then shine. Don't sleep. I think you are familiar with African people, normally due to those issues of slavery. It was like they are just waiting for the grace of God to come. Maybe they are waiting for someone to come and help them. So to me, I was thinking in different way. I said, now it is time to tell African to arise and shine. So within this organization called Arise and Shine, I have tried to solve different challenges. For example, one of the biggest challenges in African families is availability of hygiene products. Guys, you can't believe that one soap bars, we are buying one soap bar by 4,000 Tanzanian shillings up to 5,000 Tanzanian shillings. So it is very hard for those small farmholders in remote rural areas to afford this hygiene product. Is where the idea of Mama African Hygiene Solution came. I remember one day I was walking in this small town in Mbululu, looking for those smiling machines for sunflower. When you process those sunflower oil for cooking, there is those remain of sunflower sludge, do you see? So I said, according to the skills that I have obtained in my advanced studies, because in my advanced secondary school, I studied physics, chemistry, and biology. So I remember about those saponification process. Then I started to researching. After finishing my researching, I really thank God. We made a very, very good product called Mama African Soap. It is a black one to reflect the color of our moms, our girls in Africa. That's why I call them Mama Africa. So I combined all those ideas, like making skincare creams from beeswax and sunflower oil. Also making lotions from sunflower oil together with beeswax and other industrial products. So I combined all those ideas into Mama African Hygiene Solutions, do you see? So I started a partnership with small farmholders, how I can resource this raw material. Is where I made this as a project, as an empowerment project, yeah, to my community. And this is what I contacted Engineers Without Border to help me with. Again, to my informatics skills, I developed something called Bridge Gap Transparence. So I combined all my ideas. It's like I'm seeing a certain gap, then I'm coming and bridging it. Through this Bridging Gap Transparence, it has a module called Biashara Yako Mkononi Mwako. In English, we can say your business is in your hand. So it helps those small entrepreneurs, small shopkeepers, to keep their records, to track their daily sales, and keep the records of their businesses. And this is the idea. So also, this is where I contacted Engineers Without Border. And I'm really thankful for Engineers Without Border team, especially PAI. I didn't see up to date, I didn't see this lady called PAI. Is the one, the first person I contacted. And she organized all these amazing teams. Like we have Andreas, a good in software engineering. Like you, Kristoffer, in chemical engineering. So I'm really, really gladful. I'm really, really grateful to say that to have an amazing team like you guys. So that's a short story about me.

**Andreas Wendelboe** [52:42]: Yeah. Great to hear.

**Speaker 1** [52:45]: Yep.

**Andreas Wendelboe** [52:47]: I don't know if you have any questions, Martin or Kristoffer.

**Speaker 1** [52:54]: I have one regarding your soap. One thing you mentioned was that you noticed a gap between practical and theory in soap making and actually, yeah, getting these opportunities. And I was actually wondering, what do you see as the main challenges from actually knowing how to produce and make soap to actually going out and do something in the community?

**Lujo** [53:30]: Okay. Sorry, your question is?

**Speaker 1** [53:33]: My question is whether because you mentioned that there's a gap between that you see a gap between the actual theoretical knowledge and the practical knowledge of making soap. And to actually you train all these women and young people to be able to do this. But you noticed there's a gap for actually seeking these opportunities and getting an income. And I was wondering, on the long term, what challenges you think might be there for what's the reason why these women and young people can't pursue any of these opportunities?

**Lujo** [54:21]: Yeah. Yeah. What I did actually also, I do have a team here. After I come back up to my community, I organized a small group of women and youth. This is how we started. So because my passion is just to solve a certain problem and help the community. So to help the community, guys, it needs together effort. You can't work alone. For example, here in Tanzania, I am an informatics expert, do you see? So they need to have someone who, for example, I'm producing soap. So I need to have expert technical guidance, do you see? These are the challenges that we are facing right now. For example, our women and youth here, most of them, they don't have this knowledge. They don't have this skills. Maybe if you will train them, do you see? You just give them those basic, basic skills. Like this is how we are mixing. But those issues of formulation, they don't know. Even me, I don't know in a deep. So the big challenge is to have a technical guidance, a really technical guidance that will go together with national standards, together with international standards. Because I hope maybe one day we will go through all Africa, do you see? So it is across boundaries. So it needs a good foundation of technical guidance. Yeah, something like that.

**Speaker 1** [56:29]: Okay. Thank you.

**Lujo** [56:32]: Thank you.

**Speaker 1** [56:35]: Okay. Do we have any other questions?

**Andreas Wendelboe** [56:41]: I mean, I have some questions about the software stuff. I don't know if you guys want to be here for that.

**Speaker 1** [56:51]: I can still.

**Andreas Wendelboe** [56:52]: Yeah. But if we have any other questions, maybe we do that before the software stuff.

**Speaker 1** [57:01]: I just want to.

**Speaker 4** [57:02]: I have to go at 6:00, so.

**Speaker 1** [57:05]: I just want to.

**Speaker 4** [57:07]: Appreciate.

**Speaker 1** [57:07]: Yeah. Again, emphasize to please send us the list. You mentioned it already, Lujo, but can you please send a list what you have purchased, what has been the price, and also a list of brainstorm what could be potential stuff you would like to fund and to put into the project scope? Yeah.

**Lujo** [57:39]: Okay. Maybe I will send the list after this meeting, maybe.

**Speaker 1** [57:45]: Thank you. I will invite you to our meeting on the 15th of November. Yeah. Yeah. That's just what I had to say here before we end the scheduled meeting.

**Andreas Wendelboe** [58:00]: Why do we need to know the price of the stuff he bought? Is it because we want to use the funding to pay him back or how? I'm just curious from our side.

**Speaker 1** [58:12]: Maybe not the price. It's just to at least know what you bought. But I have noted it down, but just to I can also just confirm it here. It's an electrical motor, PCs, soap cutter, table, extra mold, equipment, drying rack. We can also just end it there. But at least what you would like for the future, that would be really nice if you could send us that, Lujo.

**Lujo** [58:38]: Okay. I will do that. I will send the list to you.

**Andreas Wendelboe** [58:43]: Okay. Yeah. So we don't end up placing something in the budget that you already have. That's probably a good idea, right? Kristoffer, that's why you wanted to know what you already bought.

**Speaker 1** [58:59]: Yeah. We're starting to prepare for this project scope meeting. So this is a part of the preparation.

**Andreas Wendelboe** [59:09]: Yeah. Good idea.

**Lujo** [59:10]: Yeah. So you said that we will have a meeting to discuss about the scope on November, right?

**Speaker 1** [59:19]: Yeah. I mean, we're already doing that, you can say. But this is the day where all group members will meet to write down what is the project scope, define it clear and concisely. And then we call you in at the end of the day to present what we have found. And then we can eventually ask you questions. And you can say if you agree or disagree. And also ask questions, of course.

**Lujo** [59:44]: Okay.

**Andreas Wendelboe** [59:45]: Yeah. And the scope, just to make it very clear, the scope is what is this project about? And that's what we're sending to the funds who will be giving us the money that we can give to you. So this is part of you getting funding. That is this project scope. That's why it's very important that we make it, we make it right. Because that's what we as Engineers Without Borders send to the people who give us money. We don't have any money.

**Lujo** [60:18]: Okay. So I was asking about the project scope because I was thinking about I think it is better when we are thinking about a project scope, it is better to think about regional, do you see? Because if you want to secure maybe a fund, something like that, I think it is better to invest in a regional level, do you see? For example, to empower all women and youth who found in Manyara region, do you see? Rather than district, because our district for example, the project that I'm doing right now, after maybe reaching the production of 30,000 soap bars, do you see? I am assuming that each household in Mbulu district, do you see? They can have a soap bar in a week from Mama African Hygiene Solution. So I think if we want the fund, it is better to think about regional level so that we can accommodate. Manyara region, it has five district, do you see? It has five district. Mbulu district included in Manyara region. So I'm thinking something like that.

**Andreas Wendelboe** [61:50]: Yeah. That makes sense.

**Lujo** [61:53]: Yeah.

**Speaker 1** [61:57]: Yes. Sounds good. Go ahead with your software questions, Andreas. And if.

**Speaker 5** [62:04]: I think it's just a last comment. I think it would make sense for Lujo to explain how many people we are talking about, what you just mentioned about the difference between the district and the region, what the population is, and what you would like to have the possibility to supply your soap bars to how many people. Sorry, you're muted.

**Lujo** [62:38]: Sorry. Sorry. The network was low. Sorry, can you repeat it?

**Speaker 5** [62:45]: Yeah. But I was just you mentioned the difference between district and region. And you would like to go for the region. And I think for us, it would make sense to know how many households or how many people you are talking about when you talk about region and about the production of soap bars then.

**Lujo** [63:08]: Yeah, sure. Yeah. When I'm talking about regional level, also I'm talking about the population and how many household found. For example, in roughly, I don't have the exact number. In Mbulu district, we have a population of 300,000, do you see? And all over Manyara, you can multiply it by five. You can say maybe one million population, something like that. But I don't have the exact number of yeah. Also, that population, it has household. So what I'm doing, I did a small research. For example, the annual consumption of hygiene products, especially soaps for hygiene in Mbulu district. Each household using about 45 to 48, do you see? 45 to 48 thousand Tanzanian shillings. So through Mama African Hygiene Solution, we are going to cut up that cost, and we are going to trap that market. This is a market strategy, the market strategy that I'm thinking. Yeah.

**Speaker 1** [64:37]: I didn't get the price there. What did you say? 45 to 50 thousand Tanzanian shillings for what?

**Lujo** [64:44]: For buying hygiene products like soap, like soap for laundry. The budget for each household is.

**Speaker 1** [64:55]: Per what is the time unit?

**Andreas Wendelboe** [64:58]: Per month?

**Speaker 1** [64:58]: Per year?

**Lujo** [65:00]: Oh.

**Speaker 1** [65:00]: Per year?

**Lujo** [65:01]: For year. For year.

**Speaker 1** [65:02]: For year.

**Lujo** [65:03]: Yeah.

**Speaker 1** [65:06]: Thank you.

**Lujo** [65:08]: Yeah.

**Andreas Wendelboe** [65:14]: Yeah.

**Speaker 4** [65:16]: Sorry, but I have to leave. But it was nice talking to you, Lujo, and you guys.

**Andreas Wendelboe** [65:25]: Yeah.

**Speaker 1** [65:26]: The schedule until 6:00, so no worries. Yeah.

**Speaker 4** [65:30]: Bye.

**Speaker 1** [65:31]: Bye.

**Speaker 5** [65:33]: I'm also jumping out.

**Andreas Wendelboe** [65:34]: All right.

**Lujo** [65:35]: Thank you much.

**Speaker 5** [65:36]: Thank you. Yeah. Bye-bye.

**Speaker 1** [65:38]: See you.

**Andreas Wendelboe** [65:39]: Bye.

**Speaker 1** [65:40]: Yeah. I think I'll jump out as well. See you. Thank you, Lujo.

**Lujo** [65:46]: Thank you very much, Kristoffer.

**Speaker 1** [65:48]: Thank you. Bye.

**Andreas Wendelboe** [65:49]: Yeah. All right. So are you on your laptop now, the one you bought?

**Lujo** [65:58]: Yeah. Yeah.

**Andreas Wendelboe** [65:59]: All right. And the website that you built before, did you build that without a laptop?

**Lujo** [66:14]: Yeah. I was using the desktop computer.

**Andreas Wendelboe** [66:20]: Ah, okay.

**Lujo** [66:22]: It has very, very low specifications. That's why I decided to buy a new laptop. Yeah.

**Andreas Wendelboe** [66:29]: Yeah. Good idea. And could you.

**Lujo** [66:32]: It was very, very slow. It was a dual core.

**Andreas Wendelboe** [66:35]: Oh, wow.

**Lujo** [66:37]: RAM was two, two RAMs. So I decided to buy, for example, right now, I do have Core i5. The speed of my CPU is 2.3 gigahertz. RAM, it is 8. And the storage is 256 SSD.

**Andreas Wendelboe** [67:01]: All right. And are you running Windows or Linux or what?

**Lujo** [67:08]: I'm running Windows.

**Andreas Wendelboe** [67:10]: Okay. You don't want to use the Ubuntu. There's also Ubuntu Linux, right?

**Lujo** [67:17]: Yeah. I was very lovely with the Ubuntu and Linux guy, but when I was at university. So I think I will be back again. They are very, very good in programming.

**Andreas Wendelboe** [67:32]: Yes. Yeah. I fucking hate them.

**Lujo** [67:34]: Are you using it? Are you using it?

**Andreas Wendelboe** [67:41]: My personal laptop is Linux. But my work laptop is a MacBook. But they're very similar. The terminal is almost the same. So I like both of them very much. And I absolutely hate Windows. It's a virus that should be removed.

**Lujo** [68:08]: Yeah. Sure. That is one of the biggest weakness of Windows. Yeah.

**Andreas Wendelboe** [68:15]: Yes. But could you maybe explain a little bit more about the software that you built? How did you build it? What type of what is it programmed in? What language?

**Lujo** [68:30]: Okay. Of course, I'm using Python. I'm using Python. Also, these AI tools are helping me much. And they are making me to speed up. So the language that I'm using is just Python. Also for front-end UI, user interface, I'm using Streamlight, do you see?

**Andreas Wendelboe** [69:05]: What's it called? Streamlight?

**Lujo** [69:08]: Streamlight.

**Andreas Wendelboe** [69:10]: Streamlight. Ah, okay. Yeah, yeah, yeah.

**Lujo** [69:17]: For now. And for database, because it is online, because I'm using it online and I'm testing it online, I have subscribed to Supabase. I think you are familiar with the Supabase.

**Andreas Wendelboe** [69:40]: Yeah. I know them.

**Lujo** [69:43]: Yeah. For database. But I'm not coding from scratch. What I'm doing is just using these AI tools. They are helping me to organize my idea and those source code. Then I'm putting it to the GitHub also. I'm using GitHub for displaying my source code. Yeah.

**Andreas Wendelboe** [70:09]: Yes. What AI are you using?

**Lujo** [70:19]: I'm using this I think are you familiar with cloud? Cloud.

**Andreas Wendelboe** [70:26]: Yeah. Just like cloud.

**Lujo** [70:27]: I'm fixing it. Cloud, cloud, then Gemini is found in Google. Normally, ChatGPT also is helping me to organize the idea. But for coding, normally, Gemini and cloud.

**Andreas Wendelboe** [70:45]: Okay. I'm curious. Are you using the website or are you using a program on your computer, like a CLI tool? What are you using?

**Lujo** [71:02]: For what?

**Andreas Wendelboe** [71:05]: When you're programming with AI, are you copying stuff back and forth from a website or are you using an agent on your machine?

**Lujo** [71:18]: Not an agent. I'm copying the code and I plate it. Yeah.

**Andreas Wendelboe** [71:24]: Okay. Then you have to try then you have to try coding agents. They are so much better. It's going to be a real big boost for you. Maybe I can show you.

**Lujo** [71:41]: Are they free or it needs to pay? Maybe because most of them, they need a subscription. Monthly subscription, something like that.

**Andreas Wendelboe** [71:52]: Yeah. Some you can get for free. Can you see my screen here, like my coding editor?

**Lujo** [72:05]: No.

**Andreas Wendelboe** [72:05]: No. Okay. Wait. I'll try again. I'll see if fucking hell. I'll try this. See if it works. Okay. Do you see my screen now?

**Lujo** [72:33]: Yeah. I see now.

**Andreas Wendelboe** [72:35]: Okay. So this is like normal code, right, over here. But then I have a terminal over here, like a totally normal terminal. And then what I do is I can run Claude. This also works with you can for OpenAI, you can use Codex. And for Google, you can use Gemini. And I think Gemini, you can do it for free. But I know Claude and Codex cost money. But let me just show you what you can do with it. So normally, what you would do if you were let's say you were just using Claude on website, right? Then you would start a new chat. You would maybe go here, copy all your code, put it into the chat, and then say, "I want to do blah, blah, blah, blah, blah," right?

**Lujo** [73:36]: Yeah. Yeah. Sure.

**Andreas Wendelboe** [73:37]: That's what you're doing right now, correct?

**Lujo** [73:39]: Yeah. That is what I'm doing. Yeah.

**Andreas Wendelboe** [73:41]: Yeah. So this works. But what the issue is that you need to copy in every single thing that the agent needs to know. And that's slow. So instead, what you really should do is you should have the agent running on your machine. And you can do this here. And so what I can say is I have not given it any information at all. It goes in. It looks at my repository, like the directory, sees all the files. It goes and explores all files. It looks through everything that it wants. And then it says then it comes back and returns, like, "What is this about? This is an experimental harness for a master's thesis, blah, blah, blah." Then I say, "I want to add more test cases or whatever." And then it can go out and it can edit files for you directly. So you never have to copy any code over into the thing. It runs on your machine. It reads, edits, and deletes files depending on what you need to do.

**Lujo** [75:06]: Wow. So it is going to read your files directly.

**Andreas Wendelboe** [75:11]: Yes. Yeah.

**Lujo** [75:12]: Wow.

**Andreas Wendelboe** [75:13]: So look, I have all these files here. This is a relatively big project. And it can just go and read all the necessary files. I can maybe show you yeah. So here it goes, edits files for me. I didn't have to do any of these edits at all. It can run commands for you. It can run Git. All this Git stuff, it can run those for you. It basically runs your machine for you and does coding. And it's so good at coding.

**Lujo** [76:02]: So what do you need to have so that you can do you need to install maybe something like.

**Andreas Wendelboe** [76:13]: Yeah. So you need this. I think you can do this with a free account. I'll send this in the chat, maybe. And there's also Codex, CLI. I think you can just use Codex normally. And then Claude Code.

**Lujo** [76:52]: Okay.

**Speaker 1** [76:55]: This is also new to me. CLI, is that like the desktop app or?

**Andreas Wendelboe** [77:01]: A CLI is like so you see this is a terminal, right? So a CLI is like command line interface. It's basically something that runs through your terminal. So it's not like a this is a GUI, like a graphical user interface, right? But this is something that runs only in the terminal. So that's what CLI means. So it's just a way of showing information, pretty much. Like here, then I open Claude. And then it just runs inside my terminal. I don't need to have anything open if I don't want to. Yeah.

**Lujo** [78:03]: Yeah. With these tools, so you can just command it to read your file, maybe, and ask the tool that I need this and this, something like that?

**Andreas Wendelboe** [78:18]: Yes. Exactly. Maybe I can.

**Lujo** [78:21]: But right now, all those files of Bridgegap, all those files of Bridgegap Transparence, for example, I need to reach somewhere. So I can ask it to do it?

**Andreas Wendelboe** [78:35]: Yeah. What do you mean? Like it needs to read somewhere?

**Lujo** [78:40]: For example, right now in that Biashara Yako Mkononi Mwako module, I need to have those codes to have if someone using the system and use a camera, a smartphone camera as a scanner, for example, something like that. So I need that code. I just ask the expert that, "Can you do this one?" It is something like that?

**Andreas Wendelboe** [79:07]: Yep. I can give you a demonstrator if you want. Okay. So this is a project that I'm working on in my free time. I can show you what it looks like. I built this oh, shit. I built this website. It's for tracking contracts and stuff. You have someone who wants to buy something and someone who wants to sell something. And this is like wheat and soya and all sorts of corn and stuff like that. That's the stuff that they're trading. And this is the contracts between them. And that's what I'm building right now. I'm building this whole everything has been made with these coding agents. And then I have some people that I talk to. They say, "Okay, we need this and this and this," right? They would like here. For example, then they give me they like how this works. They want it to do on something else as well. They want it to work the same way on something else. So then I basically just copy this. Then I go into my code, open Claude. Then I just say, "Implement this new feature." And then it will look through the code, find all the relevant files, find everything that's needed for it. I don't even have to look at what code it needs to know. It will find the code by itself.

**Lujo** [81:17]: Okay. Wow.

**Andreas Wendelboe** [81:21]: And then, yeah, sometimes it takes like 10 minutes or whatever because it runs a bunch of commands, right? So now it just made this change. Now it added 52 lines and deleted 10 lines from this catalog.py file. And then I have some automated testing set up where it tests I don't know if you're aware of unit tests.

**Lujo** [81:53]: Yeah. Unit tests. Yeah.

**Andreas Wendelboe** [81:55]: Yeah. So then you can see over here, it makes a new test called Test Item Page Show Who Has Worked With It From Visible Contracts, whatever. It makes the test. And then it runs the testing file to make sure that the tests pass. So it can do almost everything for you. All you have to do is plan. You have to know what you want to do and plan it. And then once you're satisfied, once you're sufficiently satisfied, you commit everything to GitHub, and then you keep building.

**Lujo** [82:37]: Wow. Wonderful. This was what I'm looking for. Wow. Wonderful.

**Andreas Wendelboe** [82:45]: Yeah. Yeah. And then what's really good is you know how you use you said you used what was it called? Streamlit, right?

**Lujo** [82:59]: Yep. Yeah.

**Andreas Wendelboe** [83:00]: To make your website. So I actually recommend AI agents are so good that you don't need this sort of can you hear me?

**Lujo** [83:15]: Yeah. Hello.

**Andreas Wendelboe** [83:16]: Yeah. Can you hear me?

**Lujo** [83:18]: Yeah. We're here.

**Andreas Wendelboe** [83:20]: Okay. Do you know this AI agents, they're so good at writing websites that you don't need Streamlit or anything like that. You don't need you can just make it in HTML and CSS. You know that stuff, right?

**Lujo** [83:41]: Yeah. HTML and CSS. Yeah. I know it.

**Andreas Wendelboe** [83:44]: Yeah. Then Claude or agents, they're so good at writing HTML, CSS, and JavaScript that you don't need a front-end framework like Streamlit. For example, this website, it doesn't use Streamlit. It doesn't use React or anything like that. It just uses pure HTML and CSS. And I just never touch it.

**Lujo** [84:17]: Expert agent.

**Andreas Wendelboe** [84:19]: Yeah.

**Lujo** [84:21]: Hello. Do you hear me?

**Andreas Wendelboe** [84:22]: Yeah. I hear you.

**Lujo** [84:25]: Yeah. I say that with this expert agent, you can maybe you can tell it, "Can you convert this Streamlit into HTML and CSS?" Right?

**Andreas Wendelboe** [84:38]: Definitely. Definitely. You would easily be able to do that. That's one of the things they're really good at. Yeah.

**Lujo** [84:47]: Wonderful. So for someone who is new and want to start, so with these links, it need me to install it to my computer or it just online?

**Andreas Wendelboe** [85:02]: Yeah. You need to install it on your computer.

**Lujo** [85:07]: Okay.

**Andreas Wendelboe** [85:08]: Yeah. And I don't know how it is on Windows. On Windows, the terminal is a little bit annoying, so it might be different for you. But I know on Linux and Mac, it's just something you run from the terminal. Maybe on Windows, you might have to download the desktop applications. This is the Claude desktop application. I'll show you here. Right. It looks like the website. But the difference is that it can make changes to your laptop or to your files and stuff. Something that's also really good, this is something called Claude Design. And you can get it to design stuff for you. You can get it to design the UI so you don't have to design it yourself. That's actually how I designed the website here. You can see this is basically what I showed you, right? And it has just designed everything for me. And the only thing I asked for I don't know if I can see. Yeah. The only thing I asked for is I'm working on this project. I'm trying to figure out how the UI, UX should be. It needs to be simple, minimalistic, easy to use, and most importantly, it look good without being too fancy. And then I gave it some info about what the project was about. It's just something I copied, pretty much. And then it worked for like 20 minutes. And then it built this thing for me.

**Lujo** [87:01]: Wow. Wonderful.

**Andreas Wendelboe** [87:04]: Yeah. So that's also something I would recommend, especially for making design. Front-end design is hard. It's hard to make it look good, make it work on mobile and stuff. You can see here, it also works on mobile devices. It looks really good.

**Lujo** [87:32]: It is flexible.

**Andreas Wendelboe** [87:34]: Yes. So yeah, I would definitely recommend that you get some of these agents. But you might need to pay money for it because they use a lot of tokens. So if you're on the free plan, then you can maybe only do stuff for like 10, 15 minutes before it says, "Okay, now you have to wait two hours before you can do it again." So for example, everyone at my work.

**Lujo** [88:15]: Everybody need it too.

**Andreas Wendelboe** [88:17]: Yeah. Yeah. Yeah. Everyone at my work I work at a software company. Everyone at my work has Claude Code. We have a plan that costs $100 a month. Every one of us has it because we use it so much.

**Lujo** [88:41]: Yeah. Sure.

**Andreas Wendelboe** [88:44]: Now, I'm not saying you need that, but maybe the $20 a month one might be worth your time, depending on how much software you make.

**Lujo** [88:56]: Yeah. Normally, I have seen that they say that 20, 25, 30. Yeah. And I can afford it. I can afford it. But I didn't know that. Normally, I just heard about AI expert agents, something like that, but I didn't experience it. Yeah.

**Andreas Wendelboe** [89:20]: Yeah. And if you can afford it, I would highly recommend Claude, getting a premium Claude account.

**Lujo** [89:32]: Wow. Yeah. Sure.

**Andreas Wendelboe** [89:34]: Because it's the best. It's the best for coding. And you can also use it for your other work in your business.

**Lujo** [89:45]: I must learn it. I must learn it, how to use it. And I normally prefer Claude because it's also producing very, very good code. No errors like others. Wonderful.

**Andreas Wendelboe** [89:59]: But does Claude have a free account, or are you paying for it?

**Lujo** [90:07]: Normally, once you use maybe two charts, three charts, it says that, "Can you upgrade it?" But they are producing very, very good codes.

**Andreas Wendelboe** [90:20]: Yes. Yeah. I would also recommend if they often say that you need to upgrade to get more stuff, I would recommend you make more chats. Every single time you're doing something new that isn't related to exactly what you were doing before, you make a new chat because then you're saving tokens. You're not using as many tokens. Because the longer the chat becomes, the quicker you use tokens. It goes really fast.

**Lujo** [90:56]: Sure. Wonderful.

**Andreas Wendelboe** [91:00]: Yeah. But yeah, I don't know. Is there anything software-wise that I can help you with? What do you need help with or support?

**Lujo** [91:20]: Yeah. Yeah. Sure. Thank you very much for this opportunity, guy. Yeah. For sure, I was just waiting for this time to come. So after getting this laptop, maybe I want to convert my because even those Streamlit is very limited. I was needing something like to have our own source code, do you see? So it's like an app, something like that. So because the way we are keep going, people maybe will need, "Can you have an app for your software?" Something like that. So what I need, the help that I the support that I need from you is maybe I can share these files, how can we convert so as it can be responsive to any devices. For example, right now, even this Streamlit, it is like a web-based, did you see? So I need something like to have our own source code. Maybe we can even we can convert it to the desktop application apps. We can convert it to the, what, Android app, APK, something like that. But with this Streamlit, I'm very, very limited. Sure.

**Andreas Wendelboe** [92:47]: Yeah. I think switching away from Streamlit over to HTML, CSS would be really good.

**Lujo** [92:55]: Yeah. Yeah. Something like that. That is what I'm looking for.

**Andreas Wendelboe** [92:59]: Yeah. If you want, you can invite me to your GitHub project.

**Lujo** [93:08]: Yeah. Sure.

**Andreas Wendelboe** [93:11]: I posted my GitHub profile here if you want to add me.

**Lujo** [93:17]: Okay. Yeah. I've seen it. Okay. Maybe also I will share mine, right? Let me share mine.

**Andreas Wendelboe** [93:38]: Yeah.

**Lujo** [93:42]: Let me open. Yeah. So I think I will have to jump out, but it's really all about this. It's so good. Yes. And it seems like it's really helpful. So I'm just glad to, yeah, get a little introduction to this.

**Andreas Wendelboe** [94:03]: Yeah.

**Lujo** [94:04]: Yeah. Kristoffer, before you leave, let me just appreciate for your time and for you to organize this meeting. I really appreciate it. Thank you very much. Blessed day.

**Andreas Wendelboe** [94:19]: You too. We will stay in touch. Yeah. Have a great evening.

**Lujo** [94:25]: Yeah. Good evening.

**Andreas Wendelboe** [94:25]: Bye-bye.

**Lujo** [94:26]: Yeah.

**Andreas Wendelboe** [94:26]: Bye.

**Lujo** [94:30]: So maybe, Andreas, maybe what we can do, maybe if I can get I don't know what time do you have. Maybe if do you have a time, maybe one hour a day in a week so that we can work on it. And I will share many things. Also, I can share the idea. For example, also, I need because one of the user who are using this, my software, said that instead of just keep searching for the product, what if I can just for example, this is a product. I just scan the QR code or something like that. Just use a camera. Then the system is automatically doing those, the other stuff. So we really need to work so that to make it very in a professional way so that we can have many users. For example, in Tanzania, if we can launch this app or software, guys, we can have a lot of subscribers. Also, I'm thinking about how we can make it for everyone who want to subscribe in our software. Maybe I can pay maybe 10,000 a month. Do you see? So the system will help. It's like a shopkeeper system. So I will share the idea, the whole idea. So I really need to work together because I have seen that you have a lot of ideas. You have a lot of tools like you have mentioned earlier. So maybe the important issue is just the plan. We can arrange me and you, our plan, maybe in this week, let accomplish this and this, something like that.

**Andreas Wendelboe** [96:41]: Yeah. When would you be available? I'm usually available in the evening after work.

**Lujo** [96:50]: Okay. Wonderful. To my side, I'm available because this is my work. I am available 24 hours because I really need to work very hard and accomplish these goals. And when we finish, it will also help. For example, that issue of the file that Kristoffer shared. Do you see? So we can use and we can input it, the good forms. It will help the groups of women. Maybe we have participated in a production in a certain area. We did one, two, three. It will also help us to show even a report to the founders, to our government, because even our government need to hear things like this. So it will be a smart way. Also, they will appreciate that we young men, we are using this, our skills to solve challenges in our communities. Also, because in Tanzania, I can say every youth in Tanzania has a smartphone nowadays. You see? So I need them to use a smartphone in very effective ways rather than just using social medias. Maybe smartphone can help people to manage their businesses, to keep records. Also, the system must help them to make decisions. We can input the intelligence part. Your business, you must maybe reduce the something like that. So we really need to have a time and discuss and set goals. Maybe in this week, let accomplish one, two, three.

**Andreas Wendelboe** [99:00]: Yeah. Yeah. That sounds good. If you can, yeah, write down your goal with the app, what do you and what things you need to build and stuff like that, we can figure out how we plan that and stuff. And then we can sit down, meet for an hour or whatever, and talk it through and maybe start working on some of the stuff.

**Lujo** [99:36]: Yeah. Sure. So you said you will be available during evening after work.

**Andreas Wendelboe** [99:43]: Yes.

**Lujo** [99:46]: And yeah. Maybe which days in a week? For example, Monday. For example, maybe Thursday. Which day is better for you?

**Andreas Wendelboe** [99:59]: Let's Monday, maybe? Maybe Monday next week.

**Lujo** [100:12]: Okay.

**Andreas Wendelboe** [100:14]: Would that work for you?

**Lujo** [100:17]: Yeah. As I said, I will be available because maybe after I accomplish all things because this is what I'm doing. So Monday.

**Andreas Wendelboe** [100:34]: Yeah.

**Lujo** [100:35]: Okay. So before Monday, I will prepare maybe what the real goal of this project, Bridge Gap Transparency, right? Then I will send you the demonstration. I will send you the link.

**Andreas Wendelboe** [100:55]: Yeah. Login, right?

**Lujo** [100:57]: Yeah. Then I will share my GitHub. Then we will start working. So I think the important issue, I will share the link, the login credential that I'm using. I will share to you. For example, right now, I don't know if do you have a time, enough time? I can share maybe.

**Andreas Wendelboe** [101:19]: Yeah. Sure.

**Lujo** [101:21]: For example, let me share the link and password. Sorry. Oh, it need me oh, my laptop is new, so it need me maybe to log in again.

**Andreas Wendelboe** [101:37]: Yeah.

**Lujo** [101:42]: Maybe I will send after this meeting. I will send because it need me maybe, yeah, to share from my phone.

**Andreas Wendelboe** [101:48]: Yeah. That's fine.

**Lujo** [101:49]: Yeah. I will share.

**Andreas Wendelboe** [101:50]: I also need to go eat dinner. So that's fine.

**Lujo** [101:54]: Yeah. Sure. Yeah.

**Andreas Wendelboe** [101:57]: But I invited you to a meeting next Monday where we can talk about software.

**Lujo** [102:06]: Oh, okay.

**Andreas Wendelboe** [102:07]: And the.

**Lujo** [102:08]: Yeah. Yeah. I have received it. Yeah.

**Andreas Wendelboe** [102:12]: Awesome.

**Lujo** [102:14]: Wonderful.

**Andreas Wendelboe** [102:16]: Yeah. And you can just send all the info you want on my WhatsApp. Yeah. Yeah. I don't think you need to do it in the group chat. You can just directly send it to my WhatsApp. That should be fine.

**Lujo** [102:32]: Yeah. This I will do in private, me and you, because this is me and you. Yeah. Maybe after we are completed this week, then we will share later on to the team, right?

**Andreas Wendelboe** [102:46]: Yeah. And I think you should consider whether you really want an app or whether it could all be on a website, but a website that was mobile-friendly. So something that could look good on a mobile screen. So I would consider whether you really need an app because if you decide to do an app, then you have to think about having it on App Store, Google Play Store, or Apple App Store. And there's a lot of then you have to submit that to them. They have to review it, and it takes time. So if all you need to do is show stuff on a phone and maybe take a photo with a camera, then I would definitely recommend only doing a website. But if you need more.

**Lujo** [103:37]: Yeah. For sure.

**Andreas Wendelboe** [103:37]: Yeah. If you need more control over the phone, you can ask AI for help with this. But if you need I would only do an app if you absolutely need the stuff that an app can do. Yeah.

**Lujo** [103:58]: Yeah. Thank you very much. Maybe I was looking for responsive web. Yeah. Something that shows that someone with a mobile phone can use it, someone with laptop, yeah, something like that.

**Andreas Wendelboe** [104:14]: Yeah. And that's very easy with AI and with HTML.

**Lujo** [104:22]: For sure. Yeah. So thank you very much, Andreas.

**Andreas Wendelboe** [104:29]: No worries.

**Lujo** [104:30]: I really appreciate. You have used a lot of time. And this is what I was looking for because I'm really excited. I need to work because the time is very, very limited. Yeah.

**Andreas Wendelboe** [104:45]: Yeah. I'm also super excited about this. I love building stuff, and I would love to understand your software and what you can do, what we can do to improve it and help people. That would be great.

**Lujo** [105:03]: Yeah. Wonderful. And I will send you the whole ideas. Where I am right now, I will send the link, everything, then you will see everything.

**Andreas Wendelboe** [105:16]: Perfect.

**Lujo** [105:18]: Yeah. Let's call it a day. Thank you very much.

**Andreas Wendelboe** [105:20]: Yeah. How do you say goodbye?

**Lujo** [105:26]: Goodbye?

**Andreas Wendelboe** [105:27]: Yeah. In your native language, how do you say goodbye?

**Lujo** [105:34]: It's normally bye-bye. Bye-bye.

**Andreas Wendelboe** [105:35]: Bye-bye. Okay.

**Lujo** [105:37]: Bye-bye. For example, bye-bye, usiku mwema.

**Andreas Wendelboe** [105:43]: Usiku mwema.

**Lujo** [105:44]: Yeah. It is good night. Bye-bye, usiku mwema.

**Andreas Wendelboe** [105:48]: Yeah. Is it Swahili or what is it?

**Lujo** [105:52]: Yeah. We are using Swahili. Swahili.

**Andreas Wendelboe** [105:54]: All right. My girlfriend speaks a little bit Swahili.

**Lujo** [106:00]: Wow. Just tell her usiku mwema. She will understand.

**Andreas Wendelboe** [106:08]: Usiku mwema. Yeah.

**Lujo** [106:10]: Yeah. Thank you very much, Andreas.

**Andreas Wendelboe** [106:14]: Yeah. Bye-bye.

**Lujo** [106:17]: Bye-bye.
