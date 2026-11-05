# Arise and Shine Software Planning

- **Date:** 2026-11-04
- **Participants:** Andreas Wendelboe, Kristoffer, Lujo
- **Source:** Wispr Flow meeting `68b4c865-7979-4633-a508-da59311b3cb3`

## Summary

Andreas Wendelboe and Kristoffer scoped a bookkeeping app for the Tanzanian soap project (MVP: production and sales records) and asked Lujo for an ambitious funding wish list before next week.

### Factory Status and Production Model
- Equipment installed over the past 2 weeks; about 300 soap bars to be produced this Friday.
- Groups can either produce at the site or order stock made by about five staff; the mix is a transition period.
    - Lujo expects groups to mostly order once familiar with the machines
- This clears up EWB confusion that groups would only buy and never produce.

### Software Scope (BridgeGap Transparent)
- Current prototype is a rough Python data-visualization mockup with community member login and Swahili production forms.
- Groups keep records in notebooks today; government also requires ingredient records (e.g. beeswax amounts).
- MVP: phone-friendly entry of production records (quantity, cost) and sales, with automatic profit calculation and alerts.
- Planned layers: group and member tracking, management dashboard for Lujo and EWB, later NGO annual reports and donor panel.

### Pricing and Money Flow
- Selling price is fixed across Mbulu District: 1 liter liquid soap sells for 2,000 Tanzanian shillings.
- Groups pay about 1,000 to 1,200 Tanzanian shillings in raw materials per liter and keep the profit.
- Organization earns its margin by reselling licensed chemicals (caustic soda) bought from suppliers.

### Funding Process and Range
- EWB volunteers prepare the proposal by 15th November; the secretariat then pitches donors, taking a couple of months.
- Upper limit is under 450,000 kroner; target range roughly 50 to 180 million Tanzanian shillings.
- Current list (website, two sheds) is too small; Andreas Wendelboe urged an ambitious plan ignoring cost.
- Volunteers usually support projects for 1 to 2 years, possibly including visits to Tanzania for in-person training.

### Needs Raised for the Plan
- Permanent empowerment hub preferred over a shipping container; possibly two sites (Mbulu town and Mbulu district).
- Training needs: bookkeeping, chemical handling, digital tools, ideally in person.
- Chemicals (three 25 kg bags of caustic soda weekly) travel by public bus from Arusha, which is illegal and dangerous.
    - A van or truck and a chemical store should go in the plan
- Kristoffer suggested also covering distribution logistics to reach distant communities.

### Next Steps
- (Lujo) Send an ambitious list of long-term needs (buildings, training, van, etc.) before next week's EWB meeting, ideally by Tuesday
- (Lujo) Send testimony videos of beneficiaries for the funding proposal
- (Lujo) Share baseline data on registered groups and beneficiaries
- (Andreas Wendelboe) Host the MVP on own server and domain before funding, migrating once funded
- (Andreas Wendelboe) Include database and domain hosting costs in the funding plan
- (Kristoffer) Help design the bookkeeping logic and database structure for the app

### Decisions Made
- MVP focuses on group production records and sales; reporting and donor dashboards come later.

## Transcript

**Lujo** [00:00]: Halløj.

**Andreas Wendelboe** [00:01]: Helo. Øhm, hvor meget— Nå, perfekt. Hello, Lujo.

**Lujo** [00:10]: Hallo. Ja, hello.

**Andreas Wendelboe** [00:14]: How are you?

**Lujo** [00:17]: I'm fine, I'm fine. How about you guys?

**Andreas Wendelboe** [00:21]: Very good.

**Lujo** [00:22]: Ja, very good, ja.

**Andreas Wendelboe** [00:25]: How's the, uh, the factory going? The setup.

**Lujo** [00:33]: Yeah, we think everything is going well. Yeah, past the 2 weeks I was busy with installing equipment, but they're working very well. Everything is good.

**Andreas Wendelboe** [00:50]: Ja. Awesome.

**Lujo** [00:51]: And this week—

**Andreas Wendelboe** [00:52]: Awesome.

**Lujo** [00:54]: —and this week, on Friday, we're expecting to do a software production. About 300 softbars at once, yeah.

**Andreas Wendelboe** [01:09]: Nice.

**Lujo** [01:10]: Ja.

**Andreas Wendelboe** [01:12]: Yeah. Um, we— so we had a meeting with the group a couple days ago, where we, like, talked about some, uh, some tasks within us, like what we would be responsible for, and what Kristoffer and I— one of our tasks was, like, figure out what to do with the software for Horizon Shine. Um, and you had shown me Bridge Gap Transparency.

**Lujo** [01:49]: Yep.

**Andreas Wendelboe** [01:50]: Um, and Kristoffer's like, it's essentially like some sort of bookkeeping. Like, it's a bookkeeping with extra functionality, I would say. Is that correct, Lujo?

**Lujo** [02:10]: Yeah, yes, it is correct.

**Andreas Wendelboe** [02:15]: Oh.

**Lujo** [02:17]: It is.

**Andreas Wendelboe** [02:18]: Yeah. Okay. And, yeah, maybe— maybe if we could, like, continue on that, like, what— what do you need from us, and, like, how can we help you from, like, a software side, a financial side? Because that's— Kristoffer's, like, he's really good at the financial stuff. Um, have you spoken to any users since we last talked?

**Lujo** [02:56]: Yeah. Uh, not yet, but— because I was very, very tight, so I didn't talk to anyone. But, uh, with my experience, you know, nowadays most people— nowadays, even in our developing countries— they have these smartphones.

**Andreas Wendelboe** [03:20]: Yeah.

**Lujo** [03:20]: Most of us, we do have these smartphones. So the main issue come is, um, how can we use these tools, yeah, to simplify our works rather than keeping records, like, in a paper form, do you see?

**Andreas Wendelboe** [03:41]: Yeah.

**Lujo** [03:42]: So it is better, maybe, for us, because we do have a knowledge in digital tools, to prepare things in a digital way, because nowadays computers are doing many things, are doing amazing work. So it is better for us to store our data, our information, in a database, you see? So the computer, it will be easier for someone. For example, let's take the example of a shopkeeper selling about 100 items, you see?

**Andreas Wendelboe** [04:23]: Mm-hmm.

**Lujo** [04:24]: If he or she wants to track one of the items, so with a system just can— with a single click, can click, maybe I'm searching for, maybe, for a pen, so the system will show the stock that you have right now with this one, yeah, and the previous selling is this one. It can show even the story, so something like that, because you will agree with me that keeping records in a paper nowadays, it is tireless, do you see? It is tireless.

**Andreas Wendelboe** [05:05]: Yeah.

**Lujo** [05:05]: Even if you want— if you want to search, to retrieve some information, it is a lot of work. Maybe you can use even a whole day in searching for items. So I think it is better, and it is good for us to develop a system that will help our user, yeah, to control their stocks, to manage their daily selling and purchases, something like that, yeah.

**Andreas Wendelboe** [05:42]: Yeah. And is there a reason they don't just use, like, some bookkeeping software that, like, already exists? Like, I know in Denmark there's, like, a ton of bookkeeping software. Um, what do they— like, I understand, like, the people who are using paper, they, of course, don't— aren't at that level yet, but, like, the people who have gone software, what are they using in, um, in Tanzania?

**Lujo** [06:19]: Uh, with my experience, also— my experience with Tanzania is, you know, even, uh, despite of a revolution of science and technology, we are— we are very, very poor in record-keeping. Even when you will ask me that, what people were using previously, what I can say is just a— is just a notebook.

**Andreas Wendelboe** [06:56]: Yeah.

**Lujo** [06:56]: Someone just use it to store, maybe, the record for a week, just to see his or her transactions. So we don't have, maybe, systems like, um, I don't have experience with that, but even when you go to the Google search right now, the software that are using in Tanzania is nowadays people are trying to develop tools that may help them. We don't have the previous, like, let's take an example of window. We were have XP Windows, then it comes UE Windows 7, something like that. But with this modern technology, this generation that come, they just have the meter with a Windows 8, Windows 10, and 11. So we don't have that revolution of software like that in our business. So even in Tanzania, what we do have is just people, they are very, very poor in record-keeping in their business. So that is one of the weakness. But nowadays people are trying to develop systems, software for stock management, for selling management, purchases management, something like that. That's why even me, I came up with, uh, with this idea that biashara yako mkononi mwako, your business in your hand. It's just an idea that you can manage your business using software, yeah.

**Andreas Wendelboe** [08:44]: Okay. And so is your idea with this, like, that it should be a general app for all types of local small businesses in Tanzania? Like, what is your target audience?

**Lujo** [09:07]: Yeah, my target is, uh, if someone has a business, maybe, because we do have different types of business, maybe vegetable businesses, shop businesses, uh, hardwares, do you see?

**Andreas Wendelboe** [09:26]: Mm-hmm.

**Lujo** [09:27]: Hardwares and other stuff. So my idea is, for example, if I'm selling pen, and I'm selling maybe, uh, and other stuffs, do you see? System may must help me with, uh, maybe I will start with registering my items, then stock in, whatever the case, maybe I don't know what shop someone owned. So I will register my items, then I will stock in, that is purchasing, I will stock in, then stock out is selling, because I'm assuming that this guy is purchasing something somewhere and stock it, then is selling. So the logic is buying, stocking.

**Andreas Wendelboe** [10:45]: Yep. Now we're back.

**Lujo** [10:47]: Hello?

**Andreas Wendelboe** [10:47]: Yeah.

**Kristoffer** [10:48]: Hello. I think your connection just—

**Andreas Wendelboe** [10:50]: Sorry, sorry.

**Lujo** [10:51]: Yeah.

**Andreas Wendelboe** [10:51]: Sorry. But we understood—

**Lujo** [10:53]: Yeah, I see my connection.

**Andreas Wendelboe** [10:56]: We understood your—

**Lujo** [10:58]: Have— yeah, have you understood the logic?

**Andreas Wendelboe** [11:01]: Yeah, yeah, yeah.

**Lujo** [11:02]: Yeah. I am assume that someone is buying something, item X, and want to sell so as to get a profit.

**Andreas Wendelboe** [11:13]: Yeah.

**Lujo** [11:14]: Do you see? So how can we going— how can we going to manage this logic?

**Andreas Wendelboe** [11:21]: Yeah. I like that idea. It's pretty interesting. I'm also curious, because this is, like, I would say that this is different from Horizon Shine, right? And it should be— it should be different. I'm curious also, like, this is something we could work on, of course, but I'm also curious from, like, Engineers Without Borders, like, this funding that we're going to be, like, applying for, for you, it might make sense if there was also something, like, related to Horizon Shine. I don't know if it is, like, for the people that will be selling the product, or if it's for some internal, like, quality assurance, stuff like this. Is there anything like that that you have been looking into, or, like, have thought about as well?

**Lujo** [12:26]: Yeah. When we come to Horizon Shine, the logic is the same. For example, maybe Group A is doing the production of soap, do you see? Then want to sell. So what is added to Horizon Shine is an issue of production, manufacturing of something. It is like purchasing, then you stock in, then you are selling. The logic is the same. I don't know if I am understandable here.

**Andreas Wendelboe** [13:06]: You are, but I'm still— I'm confused because I thought you said that these groups will not be— they will not be making the product. But are you saying that they then— they buy the product from you and then they sell it, or what? So that's the stocking and then—

**Lujo** [13:35]: Mm-hmm. In Horizon Shine, what we are doing with Project Mama African Hygiene Solution, we just trained about 30 women and youth groups, do you see? We trained leaders. So leaders can control the production of soaps, skincare creams, and lotion. For example, in our last batch production, two women were came to our production site, do you see? They came to produce organic lotion, do you see? Then they requested that to our organization that maybe tomorrow we are coming to our production site to produce organic lotion. Do you have this material? Then we granted their request that you may come because we do have raw materials. Then they are producing.

**Andreas Wendelboe** [14:43]: But—

**Lujo** [14:43]: After producing, they are going back with the product that they have for selling to their village.

**Andreas Wendelboe** [14:52]: But I'm confused because I thought you said that the youth groups and the women's groups and stuff will not be producing, that it will be your employees that will be producing. Isn't that what you've been saying these last couple of meetings?

**Lujo** [15:12]: Yeah, maybe to that previous, you were may not understand this issue. You know, because in our previous meeting you were asking that, is a whole group coming to produce? I said no. What we are planning right now to the future, rather than groups to come to use a lot of fund to travel to the production site.

**Andreas Wendelboe** [15:43]: Yeah.

**Lujo** [15:44]: They may request, because Horizon Shine have maybe five staff. Just issue of requesting that, please, can you produce this? And we are assuming that Horizon Shine we have enough stock of products already produced, rather than keep waiting for women groups to come, because it is like a bottleneck. We are using a lot of time just waiting. So I proposed that it is better for Horizon Shine to have employed staff to make production, enough stock, then if Group A requested, maybe we want 300 pieces of organic lotion, we just send to them, rather than waiting for them to come to do production, something like that.

**Andreas Wendelboe** [16:45]: Okay.

**Lujo** [16:45]: Yeah.

**Andreas Wendelboe** [16:46]: So it's, like, optional for them. They can either come to your facility and produce it, or they can buy it from you directly.

**Lujo** [17:00]: Absolutely, yeah.

**Andreas Wendelboe** [17:02]: Ah, okay. Then that has been the cause of a lot of confusion then, because we in Engineers Without Borders thought that you were moving to an exclusively them buying from you and them not producing anything. That's—

**Lujo** [17:25]: No, not exactly. It is like, for example, you are located in Denmark, I am in Tanzania, rather than for you to come from Denmark to Tanzania. It is better for you, maybe, Luciane, you know, I know how to produce organic lotion. Maybe can you produce this quantity for me? Then I said, okay, I will produce. Then take. We do have this stock. So in Horizon Shine, we are assuming that maybe we are saving 15 groups. In this month we have produced maybe 1,000 pieces of organic lotion.

**Andreas Wendelboe** [18:05]: Yeah.

**Lujo** [18:06]: So our report is going in that way. Rather than tracking maybe someone has come here to do production. It is like we are wasting time, we are wasting our resources, money, yeah.

**Andreas Wendelboe** [18:19]: Yeah. How come you're still having it, like, both ways? How come you don't go all in on the you produce it and they buy it? Why still have it, like, a mix where some people can come produce it, some people can buy it? Why not go all in on one strategy?

**Lujo** [18:48]: Yeah. To every resources that we have in Horizon Shine, for example, in our last fund that we have obtained from our friends, then we bought those equipment. So we called all those women groups leaders, and we introduced to them that, so nowadays we are going to use this equipment, do you see?

**Andreas Wendelboe** [19:15]: Mm-hmm.

**Lujo** [19:16]: So others, yeah, with their interest, they said that tomorrow I will come to the production site, I will come to produce. I said it is okay. But if you just want to request a product and I will send to you, it is okay. So it is like we are in transition, transition time.

**Andreas Wendelboe** [19:38]: Okay.

**Lujo** [19:38]: Do you see?

**Andreas Wendelboe** [19:39]: Yeah.

**Lujo** [19:39]: Because even women want to see how machine— for example, today I have received a request from two groups. Friday women will come to witness, to see how machine are producing, how that electric motor producing. So we are just allowing them to come and to see. But when they have enough experience with that, I think no one will keep coming to the production site. They will just say, please, can you send to us?

**Andreas Wendelboe** [20:15]: Okay. Because then that changes what type of software we should build, right? Because if this is just a transition period, then I suggest that we focus more on the buying from you and then selling aspect, if that makes sense.

**Lujo** [20:39]: Yep. And that is the main logic that we have. No matter, maybe even when they will come to the production site, the logic is the same. It's like they are just coming to use their energies. But the logic is still there. After you produce, the main issue is selling.

**Andreas Wendelboe** [21:02]: Yeah.

**Lujo** [21:03]: How the product will enter the market, and how profit, how you will manage the profit, and what are the impact have you made with your business, do you see? Something like that, yeah.

**Andreas Wendelboe** [21:20]: Yeah. What do you think, Kristoffer? You have any?

**Kristoffer** [21:28]: I think it sounds like a good idea to, like, keep track of it. But yeah, like, in this aspect, like, the first problem is, like, always to have, like, the right data quality. So yeah, I'm curious, how is it now with, like, people keeping record of things? Like, the women who are out selling it, are they— how are they currently keeping track of what they are selling?

**Lujo** [22:09]: Yeah. Currently we don't have— we are just keeping our records in our notebooks. Because even our government want to see what are the ingredients of this product produced. It is like how much beeswax have you put into this organic lotion, something like that. So we are just keeping to our notebooks, yeah.

**Kristoffer** [22:41]: Yeah. And then it could be a good idea to have it on your, yeah, smartphone instead, and be able to put it directly in as records, instead of, yeah, using papers. Is that something that can currently be done with the setup you have?

**Lujo** [23:10]: Yeah. In our current software that I have designed, it's just a rough— it is like a framework to show an idea. And that's why I contacted the Engineers Without Border, hoping that there is a lot of experts that will help us in this. So what I have put into the BridgeGap Transparent, we have identified our user in different groups. It is like community member groups. When you log in as a community member groups, then the system will ask you maybe the record about the production that you are. Do you see?

**Andreas Wendelboe** [24:02]: Yeah.

**Lujo** [24:04]: Yeah. Thank you, Buan Andreas, you are sharing. Yeah. In a community member group, you will see how the production of— this is a business owner. Log in as a community member group.

**Andreas Wendelboe** [24:33]: Is that the Edu or the Joyce, John?

**Lujo** [24:36]: Joyce. Joyce. Joyce. Mm-hmm.

**Andreas Wendelboe** [24:53]: So.

**Lujo** [24:55]: Taarifa za uzalishaji. Taarifa za uzalishaji.

**Andreas Wendelboe** [25:01]: Yeah. So, Kristoffer, this has been made with, like, Python and, like, basically a data visualization package for Python where you can make some websites and stuff. It's not really— it's not really built for, like, real software, like, for real websites, I would say. But it's a good, like, test to show, like, what kind of pages do you want and, yeah, what should they do, all that sort of stuff. And then.

**Kristoffer** [25:36]: Okay.

**Andreas Wendelboe** [25:36]: Just for context, I can show you— oh, actually, I had it there. This is something I showed to Lujo, like, a potential app or, like, a website design that was more, like, optimized for phones.

**Lujo** [26:02]: Mm-hmm.

**Andreas Wendelboe** [26:07]: But we have to figure out, like, okay, what type of stuff is most important for the users and, like, where to start, pretty much.

**Kristoffer** [26:20]: Yeah. I think from Engineers Without Borders perspective, one point I thought it could make sense is, like, for the whole being able to track, yeah, how many products actually reach people, because I can imagine that's part of, like, the monitoring phase of the project, being able to see that we are, like, how the impacts of what we have fundraised actually means for the people living in the region.

**Andreas Wendelboe** [26:53]: Oh, yeah.

**Lujo** [26:56]: Mm-hmm.

**Andreas Wendelboe** [26:56]: Yeah. That would be great for Engineers Without Borders so you could, like, see, okay, how many soap bars, how much lotion, how many units did you actually sell, how much profit did these groups earn. Because all this sort of stuff is perfect for Engineers Without Borders, because then we can show that to our donors and, like, show that, like, if you support Horizon Shine, then this happens in Tanzania.

**Kristoffer** [27:24]: Yeah.

**Lujo** [27:25]: Mm-hmm. Yep. Yeah. And the idea is, Buan Andreas, is maybe what we are talking right now is about it is like a shopkeeper, right? But we must add the issue of group, group members, something like that. Maybe Group A, it is called Mkombozi, for example. Do you see? This is a constitution of one of the group that I'm registering right now to Horizon Shine.

**Andreas Wendelboe** [28:00]: Yeah.

**Lujo** [28:00]: They have their constitution, and they want me to be the community development promoters. Do you see?

**Andreas Wendelboe** [28:11]: Uh-huh.

**Lujo** [28:12]: So this group have come to me to learn many things. For example, what can we do? So to the system, I can register them to the system as a group member. The activity that they are doing, maybe soap production, maybe chicken farming, something like that. So it depends about with the activities of the group. So I think it is better to add the functionality of groups. What product are they producing? What are they selling? Something like that. Does that sound good, maybe?

**Andreas Wendelboe** [28:59]: Yeah. I think— I don't know, Kristoffer, do you agree? But I think that sounds like a good, like, audience to focus on.

**Lujo** [29:12]: Mm-hmm.

**Andreas Wendelboe** [29:14]: And then we could, like— okay, so we could build, like, the website for the group members where they track buying from you and selling to others, how much did they sell for, how many units did they sell. And then behind the scenes, you, Lujo, and us, Engineers Without Borders, we have, like, a management dashboard or whatever where we can see all the groups, what they have sold, how much they sold it for, all this sort of stuff. Is that something, like, what you're envisioning?

**Lujo** [29:57]: For example, let's come back to the BridgeGap Transparent, community member login. After you log in here, maybe come here to the Taarifa za uzalishaji. Here. Yep. Do you see? The form will come— this form.

**Andreas Wendelboe** [30:26]: Yeah.

**Lujo** [30:27]: It will capture the old records about the production that they did. For example, I came to the Dongobeshe production site.

**Andreas Wendelboe** [30:36]: Yeah.

**Lujo** [30:36]: To do a liquid soap production. How much quantity we produced is here. Do you see?

**Andreas Wendelboe** [30:45]: Yeah.

**Lujo** [30:46]: The price of which. Then come to the report, report ya uzalishaji, report. Then do you see our previous production records is here.

**Andreas Wendelboe** [31:01]: Ah, yeah.

**Lujo** [31:02]: Yeah. For example, in this date, we were in Dongobeshe, we did liquid soap production, the unit price of which liquid soap is this, and we did the quantity is 60. Then the system will calculate, it will sum up. Also, there is a comment that the production went well. It is like batch production records that Kristoffer sent us.

**Andreas Wendelboe** [31:30]: Yeah.

**Lujo** [31:31]: So it is better for group to have record of this one. It will be better.

**Andreas Wendelboe** [31:37]: Yes.

**Lujo** [31:38]: Yeah. Also here, to the second button here, come here.

**Andreas Wendelboe** [31:45]: That one?

**Lujo** [31:46]: Yep. Yeah. Do you see? The system shows that the procurement of, for example, this agent called. In each group, for example, Group A, there is a member, five member. There is a Yustini, there is Alex, maybe there is Andreas. So we keep track of Yustini. What Yustini did in this month? Do you see? You will see here. The system will track what Yustini sell in that month. Do you see? Something like that.

**Andreas Wendelboe** [32:27]: Yeah.

**Lujo** [32:27]: Yeah.

**Andreas Wendelboe** [32:32]: What do you think would be the, like, the smallest— I like it has a specific name. It's called a minimal viable product. Like, what's the smallest unit of software that you could build that would be useful for these groups? Like, where should we start? What should we— the first thing that we should build that is actually useful that they would like to use?

**Lujo** [33:07]: The first one is production records.

**Andreas Wendelboe** [33:11]: Yeah.

**Lujo** [33:12]: Production records. Then, because in production we will see the cost they have used, do you see? Something like that. The quantity that they have produced. Then the selling. After production, what they sold in that period of time, the selling. Yeah.

**Andreas Wendelboe** [33:41]: Yeah. Okay. So how much— so if we could build a website where they can very easily, on their phone, enter how much did they produce, and then every time they sell something, they can enter, "I sold this and this and this for this price."

**Lujo** [33:58]: Mm-hmm.

**Andreas Wendelboe** [33:59]: If we could build that, then they would find it useful.

**Lujo** [34:09]: Then also other things, like automization of system, for example. After you get records of what you produce, what you sell, then something like profit.

**Andreas Wendelboe** [34:24]: Yeah, yeah.

**Lujo** [34:25]: That your business is in profitable way. Do you see?

**Andreas Wendelboe** [34:28]: Yeah.

**Lujo** [34:29]: Then the system will show you the alert, maybe, yeah, you are losing maybe, or you are making profit. Something like that. Automization of, yeah.

**Kristoffer** [34:43]: Can I just ask, is it a fixed price you sell the product for, or does it vary depending on where you sell it or who sells it?

**Lujo** [34:55]: Yeah. The price is fixed because right now we are operating in Mbuluh District. So the price is fixed.

**Kristoffer** [35:06]: Okay.

**Lujo** [35:06]: So, and we are based in rural areas, so we fixed the price to the, for example, one liter of liquid soap is 2,000 Tanzanian shillings. So it is fixed to whole district.

**Kristoffer** [35:24]: Ah, okay.

**Lujo** [35:26]: Yep.

**Kristoffer** [35:27]: So it's easy also. If you just get the quantity of each item, it's fairly easy to calculate how much profit you actually make from it.

**Lujo** [35:38]: Yeah.

**Andreas Wendelboe** [35:41]: And just to be very certain, you're saying the price is fixed. Is it the price that they buy it from you or that they sell it to customers?

**Lujo** [35:56]: That is a selling price.

**Andreas Wendelboe** [35:58]: Yeah.

**Lujo** [35:58]: Yeah.

**Andreas Wendelboe** [35:59]: And what about the.

**Lujo** [36:00]: Because.

**Andreas Wendelboe** [36:01]: You. They pay you something, right?

**Lujo** [36:07]: Yeah. Of course, I can say it is like they are paying, but not 100% paying me, because they are our beneficiaries. Do you see? So we share the resources. So what's important is buying raw material. So we as an organization, what we are doing is buying raw material and putting in one location, maybe in one store. Then people come and do production. For example, even me, me, I am in a group called Amka Kumekucha. Do you see? I am a CEO of this organization, Horizon Shine Excellence. But in order to be beneficiaries of this project, Mama Africa, I must abide with a certain group, for example, Amka Kumekucha. So when I go to the production site, I will enter to the production site as Amka Kumekucha group. Do you see? Then we request, in order to avoid that confusion of resources from organization to the groups. Do you see? So what we are going to do there is buying raw material, then we are producing. After producing, we are taking our product, then we are going to sell. Do you see? At the same time, our organization is earning some amount of money, do you see, by selling raw materials. And group are benefit from product. After they sell product, the profit that they are made from product is their profit.

**Kristoffer** [38:11]: So these groups pay for the raw materials to you.

**Lujo** [38:17]: Yeah.

**Kristoffer** [38:17]: Yeah. And then they get the profit from what they sell.

**Lujo** [38:23]: Yeah.

**Kristoffer** [38:24]: Okay.

**Andreas Wendelboe** [38:26]: And how much do they pay you? Is that fixed as well, the raw material price?

**Lujo** [38:32]: Yeah, it is fixed. It is fixed.

**Andreas Wendelboe** [38:35]: Okay.

**Lujo** [38:36]: For example, the cost of producing one liter of liquid soap is about 1,000 Tanzanian shillings to 1,200 Tanzanian shillings. Yeah.

**Andreas Wendelboe** [38:52]: Okay.

**Lujo** [38:53]: Yeah.

**Andreas Wendelboe** [38:53]: So they— just so I totally understand the process— they pay you 1,000 Tanzanian shillings. They come, they produce it, they take it home, they sell it for how much?

**Lujo** [39:12]: 2,000 Tanzanian shillings.

**Andreas Wendelboe** [39:15]: 2,000. Okay. And then they keep the 1,000 shillings profit.

**Lujo** [39:21]: Yeah.

**Andreas Wendelboe** [39:23]: Okay. And there's no other money moving around.

**Lujo** [39:30]: No. No.

**Andreas Wendelboe** [39:32]: Okay. Good.

**Kristoffer** [39:35]: So, but you don't take any fee to, like, keep the production site in order or for, like, maintenance or anything?

**Lujo** [39:48]: It is all included. It is all included in that. For example.

**Kristoffer** [39:53]: Okay.

**Lujo** [39:54]: Just to give you the clear clarification of this process. For example, chemicals like caustic soda, the sodium hydroxide, it is dangerous. It is not allowed for everyone to handle it. Do you see? So our organization has been given the permission of selling these chemicals. Do you see? So this is where our organization earn money.

**Andreas Wendelboe** [40:26]: Yeah.

**Lujo** [40:26]: So after we buy from a supplier, chemical supplier, then we bring to the rural areas, to our communities, then we start— we are acting like a supplier now. So the price that you are getting, for example, we are arranging our own price so as to make this project moving in a profitable way is what we are doing.

**Kristoffer** [40:54]: Okay. So you're taking a margin from the suppliers.

**Lujo** [40:59]: Yeah.

**Kristoffer** [41:00]: Yeah. Okay.

**Andreas Wendelboe** [41:05]: That should be pretty easy to do in the software to, like, since it's a pretty fixed price and pretty fixed cost, right?

**Lujo** [41:14]: Yep.

**Andreas Wendelboe** [41:21]: Yeah.

**Kristoffer** [41:25]: And, yeah, and which— where is it you need, like, help for building this? Which, yeah, which kind of skills are you in need of to be able to scale this?

**Lujo** [41:44]: Yeah. The skills that we are demanded right now is— it is like bookkeeping, right? I don't know about the business. Bookkeeping. The right way of bookkeeping to see if your business is running in a profitable way. Of course, I don't have the knowledge in bookkeeping and those language in businesses. So I think it is a knowledge that I'm needing. Also, the software engineering skills, also, I really need it because the software engineering, normally, what they are doing is looking for expert and looking for the demand and to make that logic, right? Yeah.

**Andreas Wendelboe** [42:45]: Kristoffer, do you have experience with, like, bookkeeping and all that sort of terminology and how it works?

**Kristoffer** [42:55]: Yes, I do have experience with that. And I do also actually have experience with, like, building the whole database for, like, yeah, storing it and how to structure it. So it is something, yeah, I could help with.

**Andreas Wendelboe** [43:17]: That would be great.

**Kristoffer** [43:19]: Wow. Wonderful. But I'm— yeah, but I'm thinking in terms of, like, cost, because it— like, where I can see it costs money is to have, like, the database and have, like, the domain.

**Andreas Wendelboe** [43:38]: Yeah. Hosting it.

**Kristoffer** [43:41]: Yeah.

**Andreas Wendelboe** [43:42]: Maybe that is something we should include in the funding plan.

**Kristoffer** [43:50]: Yeah. Yeah, because I'm— like, I'm for, like, hosting a domain here in Denmark, it's not really that expensive. So, yeah, it shouldn't, like, really add up, but it may also— I'm not sure if you are interested in funding it yourself, Lujo, or you'd rather have it be part of the fundraise.

**Lujo** [44:24]: Pardon?

**Kristoffer** [44:27]: Whether, like, expenses to, like, having a database and having the domain, if that's something you want to fund yourself, or if it's something you want to be part of the fundraising plan.

**Lujo** [44:44]: You know, the challenges that we are passing right now, our organization, it is still small. It is very, very small. It is like we are operating in idea, because also, the profit that we are making, it is still small. So— and we do have many, many, many things to do, because funding has been a very, very big challenge up to date, because everything that you need to do, it need money, it need money, it need money. So.

**Kristoffer** [45:25]: Yeah.

**Lujo** [45:26]: Yeah. So it is better to include in a fund, because my assumption is it is like we are doing this project just, yeah, to improve the welfare of people, communities of Tanzania. So I think it is not a good decision maybe for me to take my pocket money and do it, because I'm not doing this for business. I'm doing this just for to improve the lives of small farmer holders. Do you see?

**Kristoffer** [46:08]: Yeah.

**Lujo** [46:09]: Because here in Tanzania, many rural areas, they really, really need support. And the support that they need is knowledge, support, skills. Yeah. And to implement all this, it need funding. Do you see?

**Andreas Wendelboe** [46:25]: Yeah.

**Lujo** [46:26]: Yeah.

**Andreas Wendelboe** [46:27]: Okay. It's definitely something we can include in the funding plan.

**Kristoffer** [46:34]: Yeah.

**Lujo** [46:35]: And I don't know, because I was thinking that my assumption was engineers without border is like the engineers, they don't have a border. Maybe they can work even to Tanzania, yeah, to improve the lives of people. Something like that. It is like you are blessed with something, then you are going to use that blessing to improve lives of other. That was my assumption to engineers without border.

**Andreas Wendelboe** [47:07]: Yeah.

**Lujo** [47:07]: Yeah. But when we come to the business way, maybe I want to do this software in business way. I want to do my production site for business activity. Maybe it was— it was maybe no need for me even to look for this organization, like engineers without borders. Yeah.

**Andreas Wendelboe** [47:38]: Yeah. Kristoffer, it looked like you had a question.

**Kristoffer** [47:47]: No, it was just if it was going to be part of the fundraising, it was just then it means that to build this, it would only be built after we actually raise the money. And that would mean I have to commit to doing it after it's been fundraised.

**Andreas Wendelboe** [48:16]: I think we can do it with— we can do it before.

**Kristoffer** [48:20]: Yeah.

**Andreas Wendelboe** [48:21]: I have a server, actually, underneath my sofa.

**Kristoffer** [48:26]: Okay.

**Andreas Wendelboe** [48:27]: And I have my own domain name, so we can just set it up for Lujo just with my server and my own domain. And then once we get the funding, we can move it over to that.

**Lujo** [48:40]: Absolutely. Also.

**Kristoffer** [48:42]: That would be perfect.

**Lujo** [48:45]: Also, it is a good decision to do that, because if you have something in your hand, it will be easier for someone to support you. Yeah. Because when you represent just an idea, maybe.

**Andreas Wendelboe** [49:03]: Yeah.

**Lujo** [49:03]: For example, here in Tanzania, after doing these activities, after I presented this to my friends, people were interested and they donated me some amount of money. Even up to date, I'm here just because there is other people support. My family's members, the community members, they donated some amount of money. Even the groups.

**Andreas Wendelboe** [49:30]: Yeah.

**Lujo** [49:31]: When we come, when we want to implement something like this, for example, training, they come with a certain amount of money to pay for training. Something like that. Yeah.

**Andreas Wendelboe** [49:44]: So.

**Lujo** [49:45]: But it is better for you to start and to have hands-on.

**Andreas Wendelboe** [49:49]: Yeah.

**Lujo** [49:50]: Yeah.

**Andreas Wendelboe** [49:51]: It's easier to get funding from people once you have a product that you can show them.

**Lujo** [49:57]: Yep.

**Andreas Wendelboe** [49:59]: Okay. Kristoffer, is there anything else we need to know from Lujo before we could, like, get started with this? Like, what type of info, details do we need?

**Kristoffer** [50:19]: There was one thing I was curious about, because usually here in Denmark, companies need to provide some financial documentation every year and things like that. Is that something you do as well?

**Lujo** [50:38]: Yeah. In Tanzania, yeah, we do also, because ever after end of the year, you must represent the funding. Yeah. The funding documentation. Something like that. Yeah. Because there is a—

**Andreas Wendelboe** [50:57]: Okay.

**Lujo** [50:57]: Because in Tanzania, we do have a portal for NGOs to write your annual reports. Yeah. Quarter reports, something like that. We are doing that.

**Andreas Wendelboe** [51:12]: Okay.

**Lujo** [51:14]: Yeah.

**Kristoffer** [51:17]: Okay. It was just if the software could be used for also for providing that kind of material. I don't know if that makes sense.

**Lujo** [51:31]: Yep. Absolutely. Yeah.

**Kristoffer** [51:32]: Yeah.

**Lujo** [51:34]: Yeah. For example, it is just like I don't know, maybe Andreas, you will help me. It is like you are sharing. For example, your organization is using this system. I'm talking about the future, not now. It's just you will just share the link, then they will see the information of the activities that you are doing.

**Andreas Wendelboe** [52:01]: Yeah.

**Lujo** [52:02]: Even if you have a website, they can see the updates in your website. Something like that. Yeah.

**Kristoffer** [52:11]: Okay.

**Andreas Wendelboe** [52:14]: I definitely think it's something that's possible to do. And it would probably be a good thing to add once we have, like, the core, like the groups buying, selling, producing, whatever. Once we have that working and we have that customer or, like, users like to use it and all that sort of stuff, then we can start expanding and adding more functionality, I think.

**Lujo** [52:41]: Yep. I think maybe even when the system grow, even your donor, they want to see the progress.

**Andreas Wendelboe** [52:54]: Yeah.

**Lujo** [52:54]: You see? Then maybe we will add the donor panel. They can see the dashboard. This is what we are supporting in Tanzania. Something like that.

**Andreas Wendelboe** [53:07]: Yeah.

**Kristoffer** [53:09]: Yeah.

**Lujo** [53:09]: Yeah.

**Andreas Wendelboe** [53:10]: I think that would make it a lot easier for you to get funding and, yeah, show that you mean business.

**Lujo** [53:23]: Also, the system will help us to keep records, to track.

**Andreas Wendelboe** [53:28]: Yeah.

**Lujo** [53:28]: Rather than keep, for example, at the end of the year, rather than starting keep records. We will just come to the system. You scroll. Maybe I need the records from here to here. Also, it will be reputable to our government to see that you are working very smart. Yeah.

**Andreas Wendelboe** [53:51]: Yeah.

**Kristoffer** [53:52]: Yeah.

**Lujo** [53:55]: That is the power of database and digital systems, rather than keeping records in manual ways.

**Andreas Wendelboe** [54:06]: Yeah.

**Kristoffer** [54:07]: Definitely.

**Lujo** [54:08]: Yep.

**Andreas Wendelboe** [54:15]: Okay. I mean, I think that's pretty much all that I have questions.

**Kristoffer** [54:24]: Yeah. Me too.

**Lujo** [54:27]: Okay.

**Andreas Wendelboe** [54:32]: Yeah. I don't know if you have any closing remarks, Lujo.

**Lujo** [54:41]: I don't have, but what I can say is I just want to know from you guys, from engineers without border. You said that in 15, in November, we are going to have a scope of the project, define the scope of the project. Then after there, what else?

**Andreas Wendelboe** [55:10]: So 15th of November is when we call it project scope, but in actuality, it's like we're making the fundraising material. So we are volunteers for Engineers Without Borders, and then there's the actual organization, the secretariat, I guess you could call it. They take our, like, funding proposal, and then they go out to different people with lots of money. They go to companies and individuals and whatever and ask, "Hey, do you want to fund this exciting project in Tanzania doing soap manufacturing?" And then they'll see, like, okay, we suggest that we would like to apply for 200,000 kroner for a new shed, and we would like a new website and blah, blah, blah, blah, blah. And then they go out, they find people who want to support this, and then they get the money. And then we start the process with you where we send you the money and, like, we start helping you. And, like, I think that's pretty much as I don't know, Kristoffer, if you have other things to add.

**Kristoffer** [56:42]: No. No. I think that's basically how it works. Yeah.

**Andreas Wendelboe** [56:48]: Yeah.

**Lujo** [56:48]: Wow.

**Andreas Wendelboe** [56:49]: And I don't know how long it takes. A couple months, maybe. But I would say that, like, on the 15th of November, then that's where things really start to go fast for you, Lujo.

**Lujo** [57:09]: Oh, okay.

**Andreas Wendelboe** [57:10]: Yeah.

**Lujo** [57:11]: Because I was asking this because of, you know, every plan must have a deadline. Do you see? Something like that.

**Andreas Wendelboe** [57:21]: Yeah.

**Lujo** [57:21]: So that you can make a good decision rather than just keep waiting, keep waiting.

**Andreas Wendelboe** [57:27]: Yeah.

**Lujo** [57:27]: Yeah. Because maybe we can just use our times. At the end of the day, then we say maybe your project is not fundable. Something like that. Yeah. It is. Yeah. That's why I asked this, just to know that after.

**Andreas Wendelboe** [57:49]: Yeah.

**Lujo** [57:49]: That date, what else.

**Andreas Wendelboe** [57:52]: So.

**Lujo** [57:52]: It is like, you know, I do have experience with this American organization. Do you see?

**Andreas Wendelboe** [58:02]: Yeah.

**Lujo** [58:03]: They have their deadlines. Maybe apply before June.

**Andreas Wendelboe** [58:06]: Ah, yeah.

**Lujo** [58:07]: Blah, blah. Yeah. Something like that. Do you see?

**Andreas Wendelboe** [58:10]: Yeah.

**Lujo** [58:11]: Then after you applied, then they will give you the plan that after you apply, then you will come for interview. Then after interview, something like that.

**Andreas Wendelboe** [58:26]: Yeah. A lot of stuff.

**Lujo** [58:28]: Yeah. A lot of stuff. A lot of stuff and a lot of meeting interviews. Yeah.

**Andreas Wendelboe** [58:37]: Yeah. So what I can say is that the organization, Engineers Without Borders, definitely likes what you're doing and thinks that you are fundable.

**Lujo** [58:50]: Ah, okay.

**Andreas Wendelboe** [58:51]: But, I mean, the big question mark is, once we go and talk to people with the money, if they want to give money to us so that we can give it to you, that's a different thing. And that depends entirely on how well we, like Kristoffer, the other Kristoffer, Frieda, Martin, and everyone, like, how good we make this funding proposal. Because the better we make it, the more likely it is that donors will want to support it. But I can definitely say that Engineers Without Borders believes in this project and thinks that they can get money for it.

**Lujo** [59:45]: Wow. Wow.

**Andreas Wendelboe** [59:46]: Yeah. So.

**Lujo** [59:48]: I think it is a very, very smart way because I don't want a lot of interview.

**Andreas Wendelboe** [59:57]: Yeah.

**Lujo** [59:57]: So it is better for you to represent well our project, Mamafrika and Hygiene Solution, our organization, Rise and Shine Excellence. And now I really understand why are we keeping a lot of meeting. Yeah. I understand because you need a lot of information to us so that you will represent a good, yeah, a good idea.

**Andreas Wendelboe** [60:24]: Yeah. Yeah. So.

**Lujo** [60:25]: So I see the very, very bright futures to you guys.

**Andreas Wendelboe** [60:30]: You too.

**Lujo** [60:31]: Because it is a very, very smart way. Because if you will give me assignment to represent this to donor, I can't. I can't.

**Andreas Wendelboe** [60:45]: Yeah.

**Lujo** [60:45]: So you will help me with a lot of things.

**Andreas Wendelboe** [60:50]: Yeah. And that's the issue with a lot of, like, NGOs and funding and stuff. It's like it's so many hoops that you have to jump through to get so many interviews, so much information all the time that you have to send to them and, like, verify that it's correct. That, like, you don't have time to focus on what actually makes the difference. Like, you have to focus on fundraising instead of actually, like, helping people. And that's something I would like to avoid here as well. Like, that's also what has been annoying me about this whole process is that it has taken such a long time to get somewhere. And, like, I feel like sometimes it has been going a little bit too slow from our side, and it's maybe taking a little bit too much of your time.

**Lujo** [61:58]: Yeah.

**Andreas Wendelboe** [61:59]: But I do think that now that we have, like, all the information, that we should be able to make a good project proposal.

**Lujo** [62:09]: Yeah.

**Andreas Wendelboe** [62:09]: And I feel like we have a good understanding of what it is you do, what it is Rise and Shine wants to promote, and all this sort of stuff. All we need to really find out now is, what should we put in the funding proposal? Like, and how big should we make the funding proposal? Because I think it can be, like, I'll just convert it to Tanzanian shillings. Kristoffer, wasn't it, like, between 100 and 300,000 kroner or something like that?

**Kristoffer** [62:44]: Yeah. Less than 450,000 was the upper limit.

**Andreas Wendelboe** [62:48]: Okay. So that is 180 million Tanzanian shillings. So.

**Lujo** [63:02]: Whoa.

**Andreas Wendelboe** [63:03]: It's less than that. We think it's between 50 and 180, I would say. That's the range that we're going to be applying for.

**Lujo** [63:17]: Wow.

**Andreas Wendelboe** [63:18]: And that's why we've been asking you so many questions about, like, what do you want funded, what don't you want funded? Because the more things we can add to this funding list, the more things that make sense, the more money you can get, and the bigger difference we can make, right?

**Lujo** [63:35]: Wow. I think also, yeah, thank you very much for this information because I really didn't know how things go. Yeah. Because even when it came that the idea of a shipping container, do you see? I was just going direct to the point. This is what I was looking at for the future. I think the shipping container is not a good choice because it's just a small even we can afford in Tanzania. After six months, we can afford that. But if we will think the project in maybe for to last maybe for even 100 years. So it is better to think about to have a strong and permanent empowerment area for women, do you see, and the production site, rather than these temporary buildings.

**Andreas Wendelboe** [64:41]: Yeah.

**Lujo** [64:42]: Yeah.

**Andreas Wendelboe** [64:44]: And I think it's in your best interest to get back to us with a list of things that you would like to build or buy for the long term, like, what's for the future. Like, give a plan because it is so much money, right? It's a lot of money that you can apply for now. And I think the donors like to see this, like, plan that you want to do this and this and this. And if you get this much money, then you can do this. So I think it's good to go ambitious if we're talking this price range. And that's what we in Engineers Without Borders, we would like to be able to figure out what should we put in this funding proposal, right?

**Lujo** [65:45]: Yeah.

**Andreas Wendelboe** [65:45]: Because so far, all that we pretty much have is a website and maybe two new sheds. Like, that isn't a lot of money. So if you could send us an email or a WhatsApp listing some of the things that you could imagine you would buy in the future or build, and it's within this, like, price range of, like, 50 to 180 million Tanzanian shillings.

**Lujo** [66:21]: Yeah.

**Andreas Wendelboe** [66:22]: Then I think it would be a good idea. I don't know, Kristoffer, if you agree.

**Kristoffer** [66:29]: No. Definitely. I think that's a very good idea.

**Lujo** [66:35]: Yeah. And another issue is, you know, with that amount of money, we can make two production sites at once.

**Andreas Wendelboe** [66:49]: Yes.

**Lujo** [66:50]: For example, one in Mbulu town, Mbulu town, then and another one in Mbulu district.

**Andreas Wendelboe** [66:59]: Yes.

**Lujo** [67:00]: Yeah. Do you see? So it is something like that. Also, this issue of training, training, for example, because also I want to have, despite to have those buildings, production site buildings, you know, the experts, the skills. For example, the chemical engineering skills, something like that. Even these digital tools, skills. We need to have training, a frequent training to these groups.

**Andreas Wendelboe** [67:42]: Yeah.

**Lujo** [67:42]: Yeah.

**Andreas Wendelboe** [67:47]: I totally agree. If you could.

**Lujo** [67:49]: Yeah.

**Andreas Wendelboe** [67:51]: If you could do that before Wednesday next week.

**Lujo** [67:56]: Yeah.

**Andreas Wendelboe** [67:57]: If you could send us a list of all the things and just make it big, make it ambitious. Just the more things there are, the more things that we can look at and evaluate and consider if we should put on the funding list. Because that's been an issue for us. We've basically been trying to brainstorm, like, what can we spend money on? Like, what.

**Lujo** [68:21]: Wow. Okay.

**Andreas Wendelboe** [68:22]: That's why we asked for you to send videos of your factory or your shed so that we could understand how much, how can we improve it? What if we build 100 of these production sites? So.

**Lujo** [68:41]: You know what? Now I'm less understanding the process, how the process is going. Because before, the first email that I emailed to Engineers Without Borders, the first person I contacted, even to date, I didn't see that lady. Her name was Pai.

**Andreas Wendelboe** [69:03]: Yeah.

**Lujo** [69:04]: I included the building. I said we want an empowerment hub.

**Andreas Wendelboe** [69:10]: Yeah.

**Lujo** [69:10]: Do you see? When we say empowerment hub, you will include the building, you will include the training.

**Andreas Wendelboe** [69:18]: Yeah.

**Lujo** [69:19]: How people, for example, Kristoffer here has the skills of bookkeeping, something like that. We need to train our women groups how to bookkeeping, how to run their business in profitable ways. Do you see?

**Andreas Wendelboe** [69:32]: Yeah.

**Lujo** [69:33]: Then we do have Andreas, the software engineer. He will help us how to use systems like, "Hey, guys, we do have this app. We'll help you to do this." And then training. Do you see how empowerment, empowerment, it includes a lot of things. That is what I requested to Engineers Without Borders the first time. But after, of course, the time was passed, almost one year. After you came, then it's like we are starting afresh. It's like we are starting afresh. So I think now, at least I'm getting the understanding how the process is going. Then maybe I will send you again. I will send to the group what do we want here in Tanzania.

**Andreas Wendelboe** [70:28]: Yes.

**Lujo** [70:28]: Do you see?

**Andreas Wendelboe** [70:29]: Yes.

**Lujo** [70:30]: Yeah. Because we want to have lives like you in developed countries. So people must move smart. People must run their business in a smartest way. Do you see? Something like that. So I will do that. I will send to you.

**Andreas Wendelboe** [70:48]: Yes.

**Lujo** [70:49]: Yeah.

**Andreas Wendelboe** [70:51]: That would be great. Because, yeah, we have a meeting next week with us in Engineers Without Borders where we will, yeah, go through some of the stuff that we have figured out. So if you could send it before that meeting, ideally before, like, Tuesday or before that, then we can include it and maybe send you some questions.

**Lujo** [71:19]: Yeah. And the main issue.

**Andreas Wendelboe** [71:23]: Okay. All these questions are, it's not because we don't trust you or whatever. It's just to figure out, like, how to create the funding proposal and all that sort of stuff. That's why we're trying to figure, that's why we're asking so many questions.

**Lujo** [71:42]: And let me tell you the very, very short story. You know, in Tanzania nowadays, our government is very strict. For example, people in rural areas, they are now prevented to cut down trees, to have firewood, to have charcoal for cooking. Do you see?

**Andreas Wendelboe** [72:00]: Yeah.

**Lujo** [72:01]: At the same time, the government doesn't have alternative sources of income to its people. People are not trained. Maybe when I will stop you to cut down trees, then go and do this one. None of that issues.

**Andreas Wendelboe** [72:18]: Yeah.

**Lujo** [72:20]: We just keep seeing that. I've sent you a video about Yeda Valley Carbon Credit Fund. They want a lot of money, but that money is just going direct to the central government. But people of that area, they do not benefit about anything. But they just benefit about the maybe schools, health centers. Do you see?

**Andreas Wendelboe** [72:48]: Yeah.

**Lujo** [72:48]: But our organization, Rise and Shine Excellence, comes with a solution that, yeah, with those farm byproducts, those small farmholders, especially women and youth, are going to have an extra source of income. Do you see?

**Andreas Wendelboe** [73:09]: Yeah.

**Lujo** [73:09]: Yeah. That is a challenge that we are solving. And we do have testimony from different people. Maybe one day you will come and you will see with your naked eyes, you will see how people benefited from this. There is a lady who helped her husband to build a new house just with the money from soap making and skincare cream selling.

**Andreas Wendelboe** [73:37]: Wow.

**Lujo** [73:38]: Do you see? We do have that testimony. Yeah.

**Andreas Wendelboe** [73:41]: Can you send us that?

**Lujo** [73:41]: Do you see? Our organization. Sorry?

**Andreas Wendelboe** [73:45]: Can you send us that testimony? Because that would be great to have on the funding proposal.

**Lujo** [73:52]: Yeah. Absolutely. I will send you. I will send you a lot of videos. Do you see? That is what we are going to solve. We need to have those direct, the end user, not end user, we can say the real citizen of Tanzania.

**Andreas Wendelboe** [74:15]: Yeah.

**Lujo** [74:16]: They may benefit from the resources that we have. Because you will agree with me that our government nowadays are very, very corrupt, guys. They earn a lot of money. They earn a lot of money, but nothing. Nothing, you see. That's why we, people like me, I came with that pain direct from university. I said it is better for me to make solutions rather than keep blaming, blaming, blaming the government. Yeah.

**Andreas Wendelboe** [74:48]: Yeah.

**Lujo** [74:50]: Yeah.

**Andreas Wendelboe** [74:54]: I agree. It's a good idea.

**Lujo** [74:59]: Yeah.

**Andreas Wendelboe** [75:02]: Okay.

**Lujo** [75:02]: So I will send you the plan.

**Andreas Wendelboe** [75:04]: Yeah.

**Lujo** [75:05]: Sounds good. I will send you the plan. And also, we have baseline data right now. Since we started our activity, we do have baseline data. And we normally pick those people from poor families. Do you see? I was sorting those people. Then we register to their groups. For example, this group is called Mkombozi, Mkombozi group. Mkombozi is like a savior. So after we register them, we train them how to make soap. We train them how to make skincare creams to our production site. Then we enter into a contract with them, a contract. Then we enter the deal that I will sell you raw material. So I start to benefit from selling that raw material. The more they produce, the more our organization earning money. So this is what we are doing right now. And I will share to you. And you will get the real and the clear picture.

**Andreas Wendelboe** [76:21]: Yeah. That sounds great.

**Kristoffer** [76:23]: Very good.

**Lujo** [76:25]: Yeah.

**Andreas Wendelboe** [76:25]: I think that's all the questions that I had. I don't know, Kristoffer, if you have anything else.

**Kristoffer** [76:33]: No. Only that when you send the plan and things, you can also consider things like distribution. I'm not sure how you actually get the goods out to these communities. Is it bikes you're using that women are using to go out to the different communities? Because, yeah, I was just thinking maybe it's worth looking into, like, investing in something that can easier go long distance and things like that. So, yeah, just, yeah, think about also logistics part of it.

**Lujo** [77:19]: Yeah. Yeah. Sure. For example, one of the challenges that we are facing right now, we are sourcing our supplies from Arusha. Arusha is a long distance. If you want to transport your chemical, do you see? We are doing in very, very bad ways because we are using the public transport. Do you see? Those public buses. And our government is very, very strict that you are not allowed to transport chemical in a public transport. So if we have maybe a van, if we are going direct to Arusha to source our chemicals, then we transport in bulk.

**Andreas Wendelboe** [78:13]: Yes.

**Lujo** [78:14]: In our production site, it will be a better way because we don't have, in Tanzania, we don't have those special transport for chemicals.

**Andreas Wendelboe** [78:25]: That's 100% something that should be on the plan. You should definitely put that in the plan. Are there other problems you have like this, like big problems like this? Because you have never talked about this problem before. It's very interesting. So are there other similar-sized issues for you?

**Lujo** [78:54]: You know, my assumption before was it's like I didn't really understand the process, how the process is going. But now with this understanding, now I'm getting a clear picture. Because, you know, even when I'm thinking, it's like I'm thinking about funds. Can they afford this? When I say this, can they afford this? When I say this, can they afford it? Something like that. That's why it's like I'm very limited with vision because of funding.

**Andreas Wendelboe** [79:28]: Yeah. I think it's now you have some time, you have like a week almost to think about this, to think bigger. And then don't worry about if we can afford it because we will plan that. We will take that into consideration in the plan. And maybe we'll find like a different, cheaper alternative. So just think about, like, fuck price. Just think about what is your problem and how would you think it could be solved. And then we'll take price into consideration. Okay?

**Lujo** [80:10]: Thank you very much. And I really appreciate it because this is what I was looking for. Because even when I contacted you, I was thinking that you will help me to solve the problem. So right now, with this understanding, I'm going to think about what the problem and how to solve it.

**Andreas Wendelboe** [80:34]: Yeah. And then we'll help you fund the solution and also think of solutions. Like, it's both we provide money through our donors and we provide knowledge through us. So we'll be doing both things. So just let your mind go crazy and think of all the stuff that is problematic for you.

**Lujo** [81:07]: Yeah. Thank you. Thank you.

**Andreas Wendelboe** [81:10]: Yeah. Because.

**Lujo** [81:11]: Thank you, Ben.

**Andreas Wendelboe** [81:12]: A truck, I think, is no issue. That should be easy to include in the funding plan.

**Lujo** [81:22]: Yeah. Yeah.

**Andreas Wendelboe** [81:23]: Compared to transporting chemicals on the bus.

**Lujo** [81:29]: And we are doing that every day. And you know, we don't have a store for chemical. Again, we don't have a store for chemical. So what we did is, like, for example, normally we do have three bags of caustic soda, each with 25 kilograms, KG. Then we transport every week three bags in a public transport from Arusha to Mbulu. Then Mbulu, Mbulu to the production site found in Bashe.

**Andreas Wendelboe** [82:10]: Wow.

**Lujo** [82:10]: Again, it is a public transport. Do you see?

**Andreas Wendelboe** [82:15]: Yeah.

**Lujo** [82:15]: And sodium hydroxide is very, very dangerous. Sulfonic acid, sulfonic acid when it gets breaked, maybe, and spoil it to the people. It is very, very dangerous.

**Andreas Wendelboe** [82:34]: Yeah. It's probably good we solved that issue quickly then.

**Lujo** [82:40]: Yeah. Sure.

**Andreas Wendelboe** [82:43]: Yeah.

**Lujo** [82:43]: Yeah.

**Andreas Wendelboe** [82:44]: But okay, great. Just send us that list before next week.

**Lujo** [82:53]: And okay, sorry. And the last question maybe to you is, how about the human resource? I'm talking about, like, you volunteers. You are an expert. Then maybe when we start the project, maybe initiation of the project after make a building, is there any continuation of you to get involved to this project year after year? You volunteers?

**Andreas Wendelboe** [83:24]: Yeah. Usually, these projects, they go on for like a year or two years, I would say. So it's not just we get you funding and then we fuck off. It is we help you along the way. And sometimes a part of the funding is also so that we can travel to Tanzania and visit you and see how it works and stuff like that. So that is also something that is possible if you want that.

**Lujo** [83:57]: Yeah. That is what I was looking for because when it comes with the issue of training, we can't train them maybe visually, maybe like what we are doing, the meeting that we are doing, because our people have enough experience with person-to-person training, something like that. Maybe two weeks training, maybe you come to train about maybe bookkeeping, how to use a software, something like that. Maybe about Kristoffer, how that issue of chemical engineering, how to keep records of batch, something like that.

**Andreas Wendelboe** [84:44]: Yeah.

**Lujo** [84:44]: Yeah.

**Andreas Wendelboe** [84:46]: It's something we could do.

**Lujo** [84:49]: If we are smart enough, maybe we can use the video conferencing, a big screen, then we will keep training, visual training. It is possible. So it is possible.

**Andreas Wendelboe** [85:01]: I would love to do that.

**Lujo** [85:03]: But frequent visiting to Tanzania is much, much important.

**Andreas Wendelboe** [85:11]: Yeah.

**Lujo** [85:13]: Yeah.

**Andreas Wendelboe** [85:16]: Perfect. We'll take that into consideration and then see you again on the 15th, right?

**Lujo** [85:29]: Yeah. Yeah. So, guys, thank you very much for your time.

**Andreas Wendelboe** [85:36]: Thank you.

**Lujo** [85:36]: Thank you very much, guys. I really appreciate your time. I really appreciate your energies. Thank you very much.

**Andreas Wendelboe** [85:43]: Thank you, Lujo.

**Lujo** [85:44]: Thank you.

**Andreas Wendelboe** [85:45]: Have a good evening.

**Lujo** [85:48]: Thank you. Have a good night. Bye-bye.

**Andreas Wendelboe** [85:50]: Bye-bye.

**Kristoffer** [85:50]: Bye-bye. See you.

**Lujo** [85:53]: See you again.
