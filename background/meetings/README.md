# Arise and Shine – EWB meeting notes

Meeting summaries and transcripts from the Engineers Without Borders Denmark (Ingeniører uden grænser) soap production project in Tanzania, exported from Wispr Flow.

| Date | Meeting | Participants |
|---|---|---|
| 2026-08-25 | [Soap Production Project Kickoff](2026-08-25-soap-production-project-kickoff.md) | Sarah |
| 2026-08-25 | [Electric Mixer and TBS Certification](2026-08-25-electric-mixer-and-tbs-certification.md) | Andreas Wendelboe, Anna |
| 2026-09-08 | [Soap Production Project Planning](2026-09-08-soap-production-project-planning.md) |  |
| 2026-09-24 | [In-person project scope planning](2026-09-24-in-person-project-scope-planning.md) | Andreas Wendelboe, Frida, Martin |
| 2026-10-13 | [Engineers Without Borders Project Scope](2026-10-13-engineers-without-borders-project-scope.md) | Andreas Wendelboe, Lujo |
| 2026-10-19 | [Tanzania Data Platform Initiative](2026-10-19-tanzania-data-platform-initiative.md) | Andreas Wendelboe, Lujo |
| 2026-10-27 | [Bridge Gap Transparency Discussion](2026-10-27-bridge-gap-transparency-discussion.md) | Andreas Wendelboe |
| 2026-11-04 | [Arise and Shine Software Planning](2026-11-04-arise-and-shine-software-planning.md) | Andreas Wendelboe, Kristoffer, Lujo |

## Finding meetings in the Wispr Flow database

Wispr Flow keeps everything locally on the Mac that recorded the meeting:

```
~/Library/Application Support/Wispr Flow/
├── flow.sqlite                 ← SQLite database with meeting metadata, summaries and notes
└── meetings/<meeting-id>/
    ├── refined.ndjson          ← cleaned-up transcript (use this one)
    ├── live.ndjson             ← raw live transcript (fallback)
    └── upload.ogg              ← audio (only some meetings)
```

Open the database read-only so you don't interfere with the running app:

```bash
sqlite3 -readonly "file:$HOME/Library/Application Support/Wispr Flow/flow.sqlite?mode=ro"
```

### Tables that matter

- `Meetings`: one row per meeting. Useful columns are `id`, `title`, `createdAt`, `summary` (Markdown), `notes` (your own notes), `speakerMap` (JSON), `participantNames` and `isDeleted`.
- `Folders`: user-made folders, such as `EWB`.
- `FolderMeetings`: links folders to meetings (`folderId`, `meetingId`).

### Meetings in the EWB folder

```sql
SELECT m.id, m.createdAt, m.title
FROM FolderMeetings f
JOIN Meetings m ON m.id = f.meetingId
JOIN Folders fo ON fo.id = f.folderId
WHERE fo.name = 'EWB' AND m.isDeleted = 0
ORDER BY m.createdAt;
```

### Meetings that aren't in the folder

Not every project meeting gets filed into the EWB folder. Search titles and summaries for project keywords instead (SQLite's `LIKE` is case-insensitive for ASCII):

```sql
SELECT id, createdAt, title
FROM Meetings
WHERE isDeleted = 0 AND (
     title   LIKE '%soap%'     OR summary LIKE '%soap%'
  OR summary LIKE '%Tanzania%' OR summary LIKE '%Lujo%'
  OR summary LIKE '%EWB%'      OR summary LIKE '%Arise%'
  OR summary LIKE '%Mbulu%'    OR summary LIKE '%Engineers Without Borders%'
)
ORDER BY createdAt;
```

If the summary is empty or vague, search the transcript files directly:

```bash
cd ~/Library/Application\ Support/Wispr\ Flow/meetings
grep -il -e tanzania -e lujo -e sæbe -e soap -e mbulu */refined.ndjson
```

The folder name is the meeting `id`, which you can look up in `Meetings` to get the title and date.

### Reading a transcript

Each line of `refined.ndjson` is one utterance:

```json
{"timestamp": "00:05", "text": "...", "speaker": {"id": 1, ...}}
```

`speaker.id` is a number. To turn it into a name, look in `Meetings.speakerMap`. `assignments["<id>"]` points to a key in `people`, which has the `name`. Older meetings use the `consensus` field for this, and newer ones use `user`, `mic`, `dom` or `llm`. Take the first one that is set. Summaries refer to speakers as `<@speaker:N>` and use the same mapping. Speakers Wispr Flow couldn't identify have no mapping and appear as "Speaker N" in the exported files.
