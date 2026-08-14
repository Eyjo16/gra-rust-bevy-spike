# H01 — historical source ledger and admission gates

Date: 2026-08-14. Branch: `research/H01-source-ledger`. Base:
`2dd4db5db6f52b287ebf4f6b8a3d259bf30ba028`.

## Authority boundary

This is a **non-authoritative research ledger**, not a registry, schema,
contract, value table, or historical verdict. Its labels are workflow labels,
not additions to any closed production vocabulary. It separates four things
that must not collapse into one another:

1. a source says something;
2. an analyst infers a bounded historical proposition;
3. design proposes a mechanic inspired by that proposition;
4. the author ratifies an executable rule.

Only step 4 may authorize production meaning, and it still requires the normal
contract/registry permission and proof path. Legal prescription is not assumed
to equal everyday behavior. A late manuscript is not silently projected back
into the settlement period.

## Provisional temporal spine

These are research buckets for routing evidence, not gameplay eras or enums.

| Bucket | Approximate scope | Use | Primary caution |
|---|---|---|---|
| H01-T0 | settlement through 929 | land-taking, farm formation, pre-Commonwealth practice | later law and sagas are retrospective |
| H01-T1 | Commonwealth 930–999 | early assembly, pagan farm society | sparse contemporary text |
| H01-T2 | conversion through 1116 | Christianization and institutional transition | change may be regional and non-synchronous |
| H01-T3 | 1117–1261 | written-law culture within the Commonwealth | surviving compilations can preserve layers |
| H01-T4 | 1262–1281 and later witnesses | royal transition, late manuscripts, Járnsíða/Jónsbók | never back-project without a named argument |

The game's anchor period is still an author/lead ruling. Until it is fixed,
every mechanic claim must name the bucket(s) it purports to represent.

## Evidence ledger

Confidence is about the narrow proposition written here, not a source's general
quality. “Blocked” means no adequate source was found in this pass.

| ID | Narrow proposition | Time/place and source date | Evidence | Confidence | Permitted design use | Forbidden inference / status |
|---|---|---|---|---|---|---|
| H01-C01 | Icelandic law was long orally transmitted; a decision to write laws was made at Alþingi in 1117 and work followed in winter 1117–18 | Iceland; event 1117–18; later manuscript/history witnesses | Árnastofnun, *Grágás* and manuscript overview | high for the narrow transmission claim | model a difference between remembered/publicly spoken law and durable written record | “The surviving Grágás text is a verbatim 10th-century code” — prohibited |
| H01-C02 | Alþingi's foundation at Þingvellir in 930 marks the beginning of the old Commonwealth in the official institutional chronology; submission to Norway is dated 1262 | Iceland, 930–1262; modern official history | Alþingi history booklet | high for chronology; lower for simplified institutional detail | bound scenarios and institutional transitions to an explicit era | one timeless “Viking Iceland” ruleset — prohibited |
| H01-C03 | The two principal Grágás manuscripts and fragments are later than the first writing decision; surviving compilations can include layers and inconsistencies | mainly 12th–13th-century law/manuscripts; modern scholarship | Árnastofnun; *Some aspects of householding* | high | require material/event date and witness/manuscript date as separate fields | silently treating manuscript date, rule date, and depicted practice as identical — prohibited |
| H01-C04 | Commonwealth legal culture relied on procedure and socially situated concepts including neighbour participation, household attachment, dependants, protection, and lawful/unlawful violence | Commonwealth law as preserved later; 2024 synthesis | Miller, *Grágás and the Legal Culture of Commonwealth Iceland* | medium-high as a research map | propose separate evidence trials for witness standing, household attachment, support duties, and legal status | converting every concept into one boolean or universal numeric modifier — prohibited |
| H01-C05 | The relation between normative Grágás provisions and social practice is an interpretive problem; internal inconsistencies may preserve obsolete rules | chiefly 12th–13th centuries; 2008 article | *Some aspects of householding in the medieval Icelandic commonwealth* | high | every hard social constraint needs a practice source or an explicit “normative law only” label | law text alone proves frequency, compliance, or lived universality — prohibited |
| H01-C06 | Kin and the hrepp could bear support responsibilities, while household attachment and dependency affected legal and material position | medieval Commonwealth; preserved law analyzed in modern scholarship | householding article; Miller chapter | medium until clause-level dossier | explore obligations, dependency, and local support as conditional relationships | a universal family-affection bonus or exact support weight — unratified |
| H01-C07 | At Hofstaðir, beef and barley correlate with feasting; the study interprets feasting as an instrument of social action | Viking Age north-east Iceland; archaeology plus texts, 2013 paper | Zori et al., *Feasting in Viking Age Iceland* | high for that site/study; medium for island-wide generalization | a feast may become a costly, witnessed political action with material inputs | every feast has the same foods, status effect, or political outcome — prohibited |
| H01-C08 | Feasts and gifts could communicate power and leader-follower bonds in late-Commonwealth cases; the cited cases are unusually intense and socially narrow | late Commonwealth elite cases; 2020 chapter | Pálsson, *Forming bonds with followers* | medium-high for cases | model gift/feast consequences as audience-, relation-, and context-dependent | universal monotone loyalty gain for all people and eras — prohibited |
| H01-C09 | A 10th-century highland shieling at Pálstóftir supports summer livestock movement and also shows subsidiary craft, hunting, and possible ritual activity | eastern Iceland, 10th century; 2008 excavation study | Lucas, *Pálstóftir* | high for site; medium beyond it | seasonal production sites can host multiple activities and social meanings | every farm uses the same shieling schedule or activity mix — prohibited |
| H01-C10 | Icelandic pagan burials vary; weapons/brooches could be scarcer than horses, grave goods do not yield a simple universal status ladder, and practices changed with conversion | Icelandic Viking Age burial corpus; modern archaeological synthesis | Zori, *The Norse in Iceland* | high for variation; medium for individual interpretations | burial choices may be contextual, costly, visible acts with uncertain social readings | “weapon burial is the required honor payment” — refuted as a universal historical rule |
| H01-C11 | Religious and mortuary change should be treated as a dated transition investigated through both pagan and Christian burials, not a single instantaneous state flip | Viking Age Iceland; modern archaeological research | *Dating religious change*; Oxford synthesis | medium-high | require region/date/confession context before burial or ritual mechanics fire | applying one homogeneous pagan or Christian practice across all buckets — prohibited |
| H01-C12 | Icelandic pagan graves commonly occur singly or in small clusters near farms, routes, or boundaries rather than in urban grave fields; interpretations include territory and family land statements | Viking Age Iceland; archaeological synthesis | Zori, *The Norse in Iceland* | medium-high | candidate for witnessed memory/territory hypotheses | every burial directly creates a legal land claim — unproven |
| H01-C13 | “Four enslaved people count as one free man” | unspecified Norse/Icelandic period | no adequate Iceland-specific clause or practice source found | blocked | none | must not enter code, balance, UI fact, or lore voice |
| H01-C14 | “Spies are never free people or goðar” | unspecified Norse/Icelandic period | no adequate source found | blocked | at most an explicit counterfactual scenario hypothesis | must not become status law or NPC eligibility rule |
| H01-C15 | Burying a weapon as an honor cost | gameplay proposal, not an established historical rule | author vision plus burial scholarship showing variability | hypothesis | candidate choice for a bounded scenario after source/Meaning Gate review | never label as customary requirement or universal honor equation |
| H01-C16 | Choosing three laws every second year | gameplay proposal | author vision; no historical source in this pass | hypothesis | may be tested as a transparent strategy cadence | must not be presented as historical Alþingi procedure |
| H01-C17 | Real-time-with-pause presentation over discrete deterministic truth | product architecture | author answer, not historical evidence | author direction; implementation unratified | lead may place after discrete-time/identity law | must not be justified by historical sources |
| H01-C18 | Player occupies the current household head while the dynasty continues and other people remain free-willed | product vision | author answer, not historical evidence | author direction; semantics unratified | candidate north-star for control boundary and succession tests | must not collapse NPC agency into household ownership |

## Admission rule for a hard historical mechanic

A proposed historical constraint may enter a Meaning Gate only when its dossier
has all of the following:

- exact claim text small enough to falsify;
- geography and H01-T bucket;
- source type and author;
- date of material/event and date of surviving witness/publication;
- direct quotation locator or archaeological context, with paraphrase kept
  separate;
- whether it is normative law, observed practice, literary depiction,
  archaeological inference, or designer counterfactual;
- at least one contradiction, limitation, or missing-population warning;
- a mechanic hypothesis and a separately worded forbidden inference;
- a testable player-facing consequence;
- author verdict before values or a closed vocabulary move.

One source can support a hypothesis. A universal hard constraint requires
cross-source support appropriate to the claim and era.

## Dependency map for lead

```text
H01 source ledger
  -> H02 clause/site dossiers (one claim each)
      -> MG-H historical meaning verdicts
          -> R10 event/identity and keyed-random authority
              -> epistemic layers (fact/observation/statement/belief/judgment)
                  -> order/witness/discipline vertical slice
                      -> social/economic/personal weight trials
                          -> dynasty-scale scenario and human meaning gate
```

Historical research and engine architecture may proceed in parallel until a
mechanic makes a historical assertion. At that join, H02 and MG-H are hard
dependencies. No weight tuning begins before identity, epistemic access, legal
action space, and directional meaning are frozen.

## Rulings still needed from the author/lead

1. Anchor bucket: T0 settlement, early T1, conversion transition, mature
   Commonwealth, late Commonwealth, or an explicitly counterfactual blend?
2. Geography: all Iceland, one quarter, or a named valley/farm network?
3. Which deviations are allowed for playability, and how must the UI label
   them?
4. Is slavery in scope? If yes, which era, legal status distinctions, and
   representation constraints require specialist review?
5. Does religious change happen during play, or is confession part of the
   starting scenario?
6. Is the “three laws every second year” cadence diegetic fiction, a compressed
   strategic abstraction, or to be replaced after source review?

## Sources consulted

- [Árnastofnun — Grágás](https://www.arnastofnun.is/is/greinar/gragas)
- [Árnastofnun — Saga og bókmenntir í handritum](https://arnastofnun.is/is/saga-og-bokmenntir-i-handritum)
- [Alþingi — institutional history (PDF)](https://www.althingi.is/pdf/Althingi2017_enska.pdf)
- [Miller — Grágás and the Legal Culture of Commonwealth Iceland](https://www.cambridge.org/core/books/abs/cambridge-history-of-old-norseicelandic-literature/gragas-and-the-legal-culture-of-commonwealth-iceland/434139529B11C35DDE4C75E8FBA88CB9)
- [Some aspects of householding in the medieval Icelandic commonwealth](https://www.cambridge.org/core/journals/continuity-and-change/article/some-aspects-of-householding-in-the-medieval-icelandic-commonwealth/E0163BEFA06520CCDEF84ABFA4FFB4E7)
- [Zori et al. — Feasting in Viking Age Iceland](https://iris.hi.is/en/publications/feasting-in-viking-age-iceland-sustaining-a-chiefly-political-eco/)
- [Pálsson — Forming bonds with followers in medieval Iceland](https://iris.hi.is/en/publications/forming-bonds-with-followers-in-medieval-iceland-the-cases-of-tho/)
- [Lucas — Pálstóftir](https://iris.hi.is/is/publications/palstoftir-a-viking-age-shieling-in-iceland/)
- [Zori — The Norse in Iceland](https://academic.oup.com/edited-volume/43506/chapter/364131791)
- [Dating religious change](https://iris.hi.is/en/publications/dating-religious-change-pagan-and-christian-in-viking-age-iceland/)

## Verification

Docs-only staged tree: format and all three strict Clippy suites passed;
tests passed 56 default / 65 bevy-host / 73 bevy-render. Both feature-enabled
runtime probes exited 0 with receipts, state, and world parity true; the
frozen `10v4` envelope remained unchanged.
