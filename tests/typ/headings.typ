// Number of hashtags.

# One
### Three
###### Six

// Warning: 1:1-1:8 section depth should not exceed 6
####### Seven

---
// Heading vs. no heading.

/**/ # Heading
{[## Heading]}
[box][### Heading]

\# No heading

Text with # hashtag

Nr#1

// Error: 1:1-1:6 unexpected invalid token
#nope

---
// Heading continues over linebreak.

# This {
    "works"
}

# [box][
    This
] too

# This
does not