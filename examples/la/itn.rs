struct Intention {}

struct Idea {}

struct Movement {}

type Behav = dyn Fn(Intention) -> Idea;
