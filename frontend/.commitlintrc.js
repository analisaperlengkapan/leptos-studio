export default {
    extends: ['@commitlint/config-conventional'],
    rules: {
        'type-enum': [
            2,
            'always',
            [
                'feat',     // New feature
                'fix',      // Bug fix
                'docs',     // Documentation only changes
                'style',    // Changes that do not affect the meaning of the code
                'refactor', // Code change that neither fixes a bug nor adds a feature
                'perf',     // Performance improvement
                'test',     // Adding missing tests or correcting existing tests
                'build',    // Changes that affect the build system or external dependencies
                'ci',       // Changes to CI configuration files and scripts
                'chore',    // Other changes that don't modify src or test files
                'revert',   // Reverts a previous commit
            ],
        ],
        'subject-case': [0],  // Disabled: Allow any case (controlled by conventional-changelog)
        'subject-full-stop': [0],  // Disabled: Allow periods in commit messages
        'body-max-line-length': [0],  // Disabled: Allow longer lines in body
        'header-max-length': [2, 'always', 150],
    },
};
