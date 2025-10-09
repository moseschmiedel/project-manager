#!/bin/bash

p_cd_completions() {
    project-manager --projects-root $PROJECT_HOME list-projects
    #"Switch to project" | awk '{sub(rpl, "", $1); print $1}' rpl="$PROJECT_HOME/" | awk '{sub("/(?:.(?!/))*$", "", $0); print $0}'
}

_p_completions() {
#     commands="cd help new clone"

#     complete -c p -ef
#     complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "help" -d "Display help"
#     complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "new" -d "Create new project"
#     complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "clone" -d "Clone project with git"
#     complete -c p -n "not __fish_seen_subcommand_from $commands"  -kxa "cd" -d "Change to project dir"

# # completions for project-manager cd
#     complete -c p -n "__fish_seen_subcommand_from cd" \
#         -d "Switch project" \
#         -xa $(p_cd_completions)

# # completions for project-manager new
#     complete -c p -n "__fish_seen_subcommand_from new" -f

# # completions for project-manager clone
#     complete -c p -n "__fish_seen_subcommand_from clone" -f
#     complete -c p -n "__fish_seen_subcommand_from clone" -s p -l project-name -r -f
#     complete -c p -n "__fish_seen_subcommand_from clone" -s d -l parent-dir -r --force-files

# # completions for project-manager help
#     complete -c p -n "__fish_seen_subcommand_from help" \
#         -xa $(echo $commands) \
#         -d "Display help"
    local cur prev opts commands
    COMREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="--help --projects-root --version"

    if [[ ${cur} == -* ]]; then
        COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
        return 0
    fi

    if [ "$prev" == "p" ]; then
        commands=$(project-manager --projects-root $PROJECT_HOME list-commands)
        COMPREPLY=( $(compgen -W "$commands" -- ${cur}) )
        return 0
    fi

    if [ "$prev" == "cd" ]; then
        projects=$(project-manager --projects-root $PROJECT_HOME list-projects)
        COMPREPLY=( $(compgen -W "$projects" -- ${cur}) )
        return 0
    fi

}

complete -F _p_completions p

supported_versions="<=0.1.1"

p() {
    if [ ! project-manager supported-version "$supported_versions" ]; then
        echo "Error: project-manager version not supported. Please upgrade to a version matching $supported_versions"
        return 1
    fi

    if [ \( -n "$1" \) -a \( "$1" == "cd" \) ]; then
        cd $(project-manager --projects-root $PROJECT_HOME cd "$2");
    else
        project-manager --projects-root $PROJECT_HOME "$@"
    fi
}