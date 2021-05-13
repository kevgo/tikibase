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
        And the exit code is 1

    Scenario: fix
        When fixing
        Then it finds no issues
        And all files are unchanged

    Scenario: pitstop
        When doing a pitstop
        Then it prints:
            """
            unused image "orphan.png"
            """
        And all files are unchanged
        And the exit code is 1
