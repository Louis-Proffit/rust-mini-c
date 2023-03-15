	.text
	.globl main
print_int:
	pushq %rbp
	movq %rsp, %rbp
	addq $-16, %rsp
	movq %rdi, -8(%rbp)
	movq -8(%rbp), %r10
	movq %r10, -16(%rbp)
	movq $10, %rcx
	movq -16(%rbp), %rax
	cqto
	idivq %rax
	movq %rax, %rcx
	movq -16(%rbp), %rdi
	movq -8(%rbp), %rax
	movq $9, %r8
	cmpq %r8, %rax
	setg %al
	testq %rax, %rax
	jnz L4033
L4031:
	movq $48, %rdi
	movq -8(%rbp), %rcx
	movq $10, %r9
	movq -16(%rbp), %rax
	imulq %rax, %r9
	subq %r9, %rcx
	addq %rcx, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	movq %rbp, %rsp
	popq %rbp
	ret
L4033:
	movq -16(%rbp), %rdi
	call print_int
	addq $0, %rsp
	jmp L4031
main:
	movq $42, %rdi
	call print_int
	addq $0, %rsp
	movq $10, %rdi
	call putchar
	addq $0, %rsp
	movq $0, %rax
	ret
	.data

