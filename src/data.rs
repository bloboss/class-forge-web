//! Mock data — mirrors `CF_DATA` from the Hi-Fi v1 prototype and shadows the
//! resource models used by the Go backend at github.com/bloboss/class-forge.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ForgeKind {
    Forgejo,
    Gitlab,
    Github,
    Gitea,
    Bitbucket,
    Codeberg,
    Custom,
}

impl ForgeKind {
    pub fn slug(self) -> &'static str {
        match self {
            ForgeKind::Forgejo => "forgejo",
            ForgeKind::Gitlab => "gitlab",
            ForgeKind::Github => "github",
            ForgeKind::Gitea => "gitea",
            ForgeKind::Bitbucket => "bitbucket",
            ForgeKind::Codeberg => "codeberg",
            ForgeKind::Custom => "custom",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ForgeStatus {
    Live,
    Soon,
}

#[derive(Clone, Debug)]
pub struct Forge {
    pub id: &'static str,
    pub kind: ForgeKind,
    pub label: &'static str,
    pub status: ForgeStatus,
    pub org: Option<&'static str>,
    pub accounts: u32,
    pub note: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CoverKind {
    Indigo,
    Warm,
    Slate,
    Rose,
}

#[derive(Clone, Debug)]
pub struct UpcomingAssignment {
    pub name: &'static str,
    pub due_in: &'static str,
}

#[derive(Clone, Debug)]
pub struct Classroom {
    pub id: &'static str,
    pub number: &'static str,
    pub name: &'static str,
    pub term: &'static str,
    pub forge: &'static str,
    pub students: u32,
    pub teams: u32,
    pub assignments_count: u32,
    pub active_assignments: u32,
    pub cover: CoverKind,
    pub next_due: UpcomingAssignment,
    pub pending_push: u32,
    pub passing: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssignmentKind {
    Individual,
    Team,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AssignmentStatus {
    Active,
    Graded,
    Draft,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub id: &'static str,
    pub class_id: &'static str,
    pub title: &'static str,
    pub kind: AssignmentKind,
    pub team_size: Option<(u32, u32)>,
    pub template: &'static str,
    pub branch: &'static str,
    pub assigned: &'static str,
    pub due: &'static str,
    pub due_in: &'static str,
    pub status: AssignmentStatus,
    pub accepted: u32,
    pub total: u32,
    pub passing_tests: f32,
    pub submissions: u32,
    pub late: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RosterStatus {
    Linked,
    Pending,
    Invited,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RosterRole {
    Student,
    Ta,
}

#[derive(Clone, Debug)]
pub struct RosterEntry {
    pub id: &'static str,
    pub name: &'static str,
    pub sid: &'static str,
    pub email: &'static str,
    pub section: &'static str,
    pub role: RosterRole,
    pub forge: &'static str,
    pub handle: Option<&'static str>,
    pub status: RosterStatus,
    pub grade: Option<f32>,
    pub last_push: &'static str,
}

#[derive(Clone, Debug)]
pub struct User {
    pub name: &'static str,
    pub email: &'static str,
    pub handle: &'static str,
    pub org: &'static str,
    pub initials: &'static str,
}

pub fn user() -> User {
    User {
        name: "Dr. Mira Acharya",
        email: "macharya@univ.edu",
        handle: "macharya",
        org: "Pacific Coast University · CS Dept",
        initials: "MA",
    }
}

pub fn forges() -> Vec<Forge> {
    use ForgeKind::*;
    use ForgeStatus::*;
    vec![
        Forge {
            id: "forgejo-univ",
            kind: Forgejo,
            label: "Forgejo · forge.cs.pcu.edu",
            status: Live,
            org: Some("cs-dept"),
            accounts: 4,
            note: "self-hosted",
        },
        Forge {
            id: "gitlab-cloud",
            kind: Gitlab,
            label: "GitLab Cloud",
            status: Live,
            org: Some("pcu-classes"),
            accounts: 2,
            note: "",
        },
        Forge {
            id: "github",
            kind: Github,
            label: "GitHub",
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q3 2026",
        },
        Forge {
            id: "gitea",
            kind: Gitea,
            label: "Gitea",
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q3 2026",
        },
        Forge {
            id: "bitbucket",
            kind: Bitbucket,
            label: "Bitbucket",
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q4 2026",
        },
        Forge {
            id: "codeberg",
            kind: Codeberg,
            label: "Codeberg",
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q4 2026",
        },
        Forge {
            id: "custom",
            kind: Custom,
            label: "Custom (federated)",
            status: Soon,
            org: None,
            accounts: 0,
            note: "preview",
        },
    ]
}

pub fn classes() -> Vec<Classroom> {
    use CoverKind::*;
    vec![
        Classroom {
            id: "cs331",
            number: "CS 331",
            name: "Algorithms & Complexity",
            term: "Spring 2026",
            forge: "forgejo-univ",
            students: 142,
            teams: 0,
            assignments_count: 8,
            active_assignments: 2,
            cover: Indigo,
            next_due: UpcomingAssignment {
                name: "Hash maps & amortized analysis",
                due_in: "in 3 days",
            },
            pending_push: 12,
            passing: 0.74,
        },
        Classroom {
            id: "cs214",
            number: "CS 214",
            name: "Software Engineering",
            term: "Spring 2026",
            forge: "gitlab-cloud",
            students: 98,
            teams: 24,
            assignments_count: 5,
            active_assignments: 1,
            cover: Warm,
            next_due: UpcomingAssignment {
                name: "Sprint 3 retrospective",
                due_in: "in 6 days",
            },
            pending_push: 4,
            passing: 0.81,
        },
        Classroom {
            id: "cs101",
            number: "CS 101",
            name: "Introduction to Programming",
            term: "Spring 2026",
            forge: "forgejo-univ",
            students: 318,
            teams: 0,
            assignments_count: 12,
            active_assignments: 3,
            cover: Slate,
            next_due: UpcomingAssignment {
                name: "Lab 7 — Recursion warm-ups",
                due_in: "tomorrow",
            },
            pending_push: 41,
            passing: 0.69,
        },
        Classroom {
            id: "cs560",
            number: "CS 560",
            name: "Distributed Systems (grad)",
            term: "Spring 2026",
            forge: "gitlab-cloud",
            students: 36,
            teams: 9,
            assignments_count: 4,
            active_assignments: 1,
            cover: Rose,
            next_due: UpcomingAssignment {
                name: "Raft project — milestone 2",
                due_in: "in 11 days",
            },
            pending_push: 2,
            passing: 0.86,
        },
    ]
}

pub fn assignments() -> Vec<Assignment> {
    use AssignmentKind::*;
    use AssignmentStatus::*;
    vec![
        Assignment {
            id: "hw04",
            class_id: "cs331",
            title: "Homework 4 — Hash maps & amortized analysis",
            kind: Individual,
            team_size: None,
            template: "cs331-staff/hw04-template",
            branch: "main",
            assigned: "Apr 24",
            due: "May 13, 11:59pm",
            due_in: "in 3 days",
            status: Active,
            accepted: 138,
            total: 142,
            passing_tests: 0.62,
            submissions: 124,
            late: 3,
        },
        Assignment {
            id: "hw03",
            class_id: "cs331",
            title: "Homework 3 — Heaps, priority queues, Dijkstra",
            kind: Individual,
            team_size: None,
            template: "cs331-staff/hw03-template",
            branch: "main",
            assigned: "Apr 10",
            due: "Apr 24, 11:59pm",
            due_in: "closed",
            status: Graded,
            accepted: 141,
            total: 142,
            passing_tests: 0.79,
            submissions: 141,
            late: 7,
        },
        Assignment {
            id: "proj01",
            class_id: "cs331",
            title: "Project 1 — Build a key-value store",
            kind: Team,
            team_size: Some((2, 3)),
            template: "cs331-staff/proj1-kv-template",
            branch: "main",
            assigned: "Apr 28",
            due: "May 22, 11:59pm",
            due_in: "in 12 days",
            status: Active,
            accepted: 41,
            total: 48,
            passing_tests: 0.34,
            submissions: 22,
            late: 0,
        },
        Assignment {
            id: "hw02",
            class_id: "cs331",
            title: "Homework 2 — Sorting & divide-and-conquer",
            kind: Individual,
            team_size: None,
            template: "cs331-staff/hw02-template",
            branch: "main",
            assigned: "Mar 27",
            due: "Apr 10, 11:59pm",
            due_in: "closed",
            status: Graded,
            accepted: 140,
            total: 142,
            passing_tests: 0.84,
            submissions: 140,
            late: 4,
        },
        Assignment {
            id: "hw01",
            class_id: "cs331",
            title: "Homework 1 — Big-O warm-up",
            kind: Individual,
            team_size: None,
            template: "cs331-staff/hw01-template",
            branch: "main",
            assigned: "Mar 13",
            due: "Mar 27, 11:59pm",
            due_in: "closed",
            status: Graded,
            accepted: 142,
            total: 142,
            passing_tests: 0.91,
            submissions: 142,
            late: 1,
        },
        Assignment {
            id: "draft01",
            class_id: "cs331",
            title: "Final exam — practice problems",
            kind: Individual,
            team_size: None,
            template: "—",
            branch: "main",
            assigned: "—",
            due: "—",
            due_in: "draft",
            status: Draft,
            accepted: 0,
            total: 142,
            passing_tests: 0.0,
            submissions: 0,
            late: 0,
        },
    ]
}

pub fn roster() -> Vec<RosterEntry> {
    use RosterRole::*;
    use RosterStatus::*;
    vec![
        RosterEntry {
            id: "s001",
            name: "Aanya Patel",
            sid: "904112038",
            email: "apatel@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("apatel"),
            status: Linked,
            grade: Some(0.92),
            last_push: "2h ago",
        },
        RosterEntry {
            id: "s002",
            name: "Beatriz Oliveira",
            sid: "904128201",
            email: "boliveira@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("beatrizo"),
            status: Linked,
            grade: Some(0.88),
            last_push: "5h ago",
        },
        RosterEntry {
            id: "s003",
            name: "Cyrus Mehta",
            sid: "904122014",
            email: "cmehta@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("cyrus"),
            status: Linked,
            grade: Some(0.71),
            last_push: "1d ago",
        },
        RosterEntry {
            id: "s004",
            name: "Daniela Ruiz",
            sid: "904117734",
            email: "druiz@pcu.edu",
            section: "B",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("daniruiz"),
            status: Linked,
            grade: Some(0.64),
            last_push: "9d ago",
        },
        RosterEntry {
            id: "s005",
            name: "Ezra Cohen",
            sid: "904112987",
            email: "ecohen@pcu.edu",
            section: "B",
            role: Student,
            forge: "forgejo-univ",
            handle: None,
            status: Pending,
            grade: None,
            last_push: "—",
        },
        RosterEntry {
            id: "s006",
            name: "Felicia Wong",
            sid: "904122441",
            email: "fwong@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("fwong"),
            status: Linked,
            grade: Some(0.95),
            last_push: "1h ago",
        },
        RosterEntry {
            id: "s007",
            name: "Gabriel Asare",
            sid: "904118843",
            email: "gasare@pcu.edu",
            section: "B",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("gabea"),
            status: Linked,
            grade: Some(0.58),
            last_push: "11d ago",
        },
        RosterEntry {
            id: "s008",
            name: "Hadley Park",
            sid: "904132002",
            email: "hpark@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("hpark"),
            status: Linked,
            grade: Some(0.79),
            last_push: "3h ago",
        },
        RosterEntry {
            id: "s009",
            name: "Imran Saleh",
            sid: "904142919",
            email: "isaleh@pcu.edu",
            section: "B",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("imran"),
            status: Linked,
            grade: Some(0.83),
            last_push: "yesterday",
        },
        RosterEntry {
            id: "s010",
            name: "Jordan Bailey",
            sid: "904112700",
            email: "jbailey@pcu.edu",
            section: "B",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("jbailey"),
            status: Linked,
            grade: Some(0.67),
            last_push: "4d ago",
        },
        RosterEntry {
            id: "s011",
            name: "Kai Yamamoto",
            sid: "904118203",
            email: "kyamamoto@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: Some("kyamamoto"),
            status: Linked,
            grade: Some(0.74),
            last_push: "8h ago",
        },
        RosterEntry {
            id: "s012",
            name: "Lila Andersson",
            sid: "904192223",
            email: "landersson@pcu.edu",
            section: "A",
            role: Student,
            forge: "forgejo-univ",
            handle: None,
            status: Invited,
            grade: None,
            last_push: "—",
        },
        RosterEntry {
            id: "t001",
            name: "Sasha Volkov",
            sid: "—",
            email: "svolkov@pcu.edu",
            section: "—",
            role: Ta,
            forge: "forgejo-univ",
            handle: Some("svolkov"),
            status: Linked,
            grade: None,
            last_push: "30m ago",
        },
        RosterEntry {
            id: "t002",
            name: "Marcus Lin",
            sid: "—",
            email: "mlin@pcu.edu",
            section: "—",
            role: Ta,
            forge: "forgejo-univ",
            handle: Some("mlin"),
            status: Linked,
            grade: None,
            last_push: "1h ago",
        },
    ]
}

pub fn forge_by_id(id: &str) -> Option<Forge> {
    forges().into_iter().find(|f| f.id == id)
}

pub fn class_by_id(id: &str) -> Option<Classroom> {
    classes().into_iter().find(|c| c.id == id)
}

pub fn assignments_for_class(class_id: &str) -> Vec<Assignment> {
    assignments()
        .into_iter()
        .filter(|a| a.class_id == class_id)
        .collect()
}

pub fn fmt_pct(n: f32) -> String {
    format!("{}%", (n * 100.0).round() as i32)
}

pub fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}
