Feature: orphaned resource

    Background:
        Given file "orphan.png"

    Scenario: check
        When checking
        Then it prints:
            """
            file "orphan.png" isn't linked to
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
            file "orphan.png" isn't linked to
            """
        And all files are unchanged
        And the exit code is 1
