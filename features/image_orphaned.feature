Feature: orphaned resource

    Background:
        Given binary file "orphan.png"

    Scenario: this
        When checking
        Then it prints:
            """
            unused image "image.png"
            """
        And all files are unchanged
