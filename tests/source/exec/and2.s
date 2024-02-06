	.text
	.globl main
main:
	movq $65, %rdi
	movq $0, %rdx
	testq %rdx, %rdx
	jz L93
	movq $1, %rdx
	testq %rdx, %rdx
	jz L93
	movq $1, %rdx
L93:
	addq %rdx, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rcx
	testq %rcx, %rcx
	jz L81
	movq $2, %rcx
	testq %rcx, %rcx
	jz L81
	movq $1, %rcx
L81:
	addq %rcx, %rdi
	call putchar
	movq $65, %rdi
	movq $1, %r8
	testq %r8, %r8
	jz L59
	movq $0, %r8
	testq %r8, %r8
	jz L59
	movq $1, %r8
L59:
	addq %r8, %rdi
	call putchar
	movq $65, %rdi
	movq $0, %rax
	testq %rax, %rax
	jz L37
	movq $0, %rax
	testq %rax, %rax
	jz L37
	movq $1, %rax
L37:
	addq %rax, %rdi
	call putchar
	movq $10, %rdi
	call putchar
	movq $0, %rax
	ret
	jmp L37
	jmp L37
	jmp L59
	jmp L59
	jmp L81
	jmp L81
	jmp L93
	jmp L93
	.data

