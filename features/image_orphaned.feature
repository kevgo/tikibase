Feature: orphaned resource

    Background:
        Given binary file "orphan.png"

    Scenario: this
        When checking
        Then it prints:
            """
            unused image "orphan.png"
            """
        And all files are unchanged
