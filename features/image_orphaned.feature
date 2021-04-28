Feature: orphaned resource

    Background:
        Given binary file "orphan.png"

    Scenario: check
        When checking
        Then it prints:
            """
            unused image "orphan.png"
            """
        And all files are unchanged

    Scenario: pitstop
        When doing a pitstop
        Then it prints:
            """
            unused image "orphan.png"
            """
        And all files are unchanged
